use crc32fast::Hasher;

const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
const JPEG_SOI: &[u8; 2] = b"\xff\xd8";
const ICC_JPEG_MARKER: &[u8] = b"ICC_PROFILE\0";
const SAMPLE_PNG: &[u8] = include_bytes!("../assets/images/original.png");

#[test]
fn selects_icc_by_type() {
    assert_eq!(sdr2hdr::icc::by_type(1).unwrap(), sdr2hdr::icc::icc1());
    assert_eq!(sdr2hdr::icc::by_type(2).unwrap(), sdr2hdr::icc::icc2());
    assert!(sdr2hdr::icc::by_type(0).is_err());
}

#[test]
fn embeds_icc_into_sample_png_file() {
    let output = sdr2hdr::embed_icc(SAMPLE_PNG, sdr2hdr::icc::icc1()).unwrap();
    let chunks = read_png_chunks(&output);

    assert_eq!(chunks[0].name, *b"IHDR");
    assert!(chunks.iter().any(|chunk| chunk.name == *b"iCCP"));
    assert_eq!(
        chunks.iter().filter(|chunk| chunk.name == *b"iCCP").count(),
        1
    );
}

#[test]
fn replaces_existing_png_iccp_chunk() {
    let mut png = Vec::from(PNG_SIGNATURE.as_slice());
    write_png_chunk(&mut png, b"IHDR", &[0; 13]);
    write_png_chunk(&mut png, b"iCCP", b"old");
    write_png_chunk(&mut png, b"IDAT", &[1, 2, 3]);
    write_png_chunk(&mut png, b"IEND", &[]);

    let output = sdr2hdr::embed_icc(&png, b"test profile").unwrap();
    let chunks = read_png_chunks(&output);

    assert_eq!(chunks[0].name, *b"IHDR");
    assert_eq!(chunks[1].name, *b"iCCP");
    assert_eq!(chunks[2].name, *b"IDAT");
    assert_eq!(
        chunks.iter().filter(|chunk| chunk.name == *b"iCCP").count(),
        1
    );
}

#[test]
fn replaces_existing_jpeg_icc_segment() {
    let jpeg = [
        0xff, 0xd8, 0xff, 0xe2, 0x00, 0x14, b'I', b'C', b'C', b'_', b'P', b'R', b'O', b'F', b'I',
        b'L', b'E', 0, 1, 1, b'o', b'l', b'd', b'!', 0xff, 0xda, 0x00, 0x0c, 1, 2, 3, 4,
    ];
    let output = sdr2hdr::embed_icc(&jpeg, b"test profile").unwrap();

    assert_eq!(&output[..2], JPEG_SOI);
    assert_eq!(&output[2..4], &[0xff, 0xe2]);
    assert!(
        output
            .windows(ICC_JPEG_MARKER.len())
            .any(|window| window == ICC_JPEG_MARKER)
    );
    assert!(!output.windows(4).any(|window| window == b"old!"));
}

#[derive(Clone, Copy)]
struct PngChunk<'a> {
    name: [u8; 4],
    _data: &'a [u8],
}

fn read_png_chunks(image: &[u8]) -> Vec<PngChunk<'_>> {
    assert!(image.starts_with(PNG_SIGNATURE));

    let mut chunks = Vec::new();
    let mut offset = PNG_SIGNATURE.len();

    while offset + 12 <= image.len() {
        let length = u32::from_be_bytes(image[offset..offset + 4].try_into().unwrap()) as usize;
        let name_offset = offset + 4;
        let data_offset = name_offset + 4;
        let crc_offset = data_offset + length;
        let next_offset = crc_offset + 4;

        let mut name = [0; 4];
        name.copy_from_slice(&image[name_offset..data_offset]);
        chunks.push(PngChunk {
            name,
            _data: &image[data_offset..crc_offset],
        });

        offset = next_offset;
        if name == *b"IEND" {
            break;
        }
    }

    chunks
}

fn write_png_chunk(output: &mut Vec<u8>, name: &[u8; 4], data: &[u8]) {
    output.extend_from_slice(&(data.len() as u32).to_be_bytes());
    output.extend_from_slice(name);
    output.extend_from_slice(data);

    let mut hasher = Hasher::new();
    hasher.update(name);
    hasher.update(data);
    output.extend_from_slice(&hasher.finalize().to_be_bytes());
}
