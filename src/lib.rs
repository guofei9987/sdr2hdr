use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crc32fast::Hasher;
use flate2::{Compression, write::ZlibEncoder};

const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
const JPEG_SOI: &[u8; 2] = b"\xff\xd8";
const ICC_JPEG_MARKER: &[u8] = b"ICC_PROFILE\0";
const JPEG_ICC_PAYLOAD_MAX: usize = 65_519;

pub mod icc {
    pub const ICC1: &[u8] = include_bytes!("icc/icc1.icc");
    pub const ICC2: &[u8] = include_bytes!("icc/icc2.icc");

    pub fn icc1() -> &'static [u8] {
        ICC1
    }

    pub fn icc2() -> &'static [u8] {
        ICC2
    }
}

pub fn read_icc(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    fs::read(path)
}

pub fn embed_icc_file(
    input_image: impl AsRef<Path>,
    icc_file: impl AsRef<Path>,
    output_image: impl AsRef<Path>,
) -> io::Result<()> {
    let image = fs::read(input_image.as_ref())?;
    let icc = read_icc(icc_file)?;
    let output = embed_icc(&image, &icc)?;
    fs::write(output_image, output)
}

pub fn embed_icc(image: &[u8], icc: &[u8]) -> io::Result<Vec<u8>> {
    if image.starts_with(PNG_SIGNATURE) {
        return embed_png_icc(image, icc);
    }

    if image.starts_with(JPEG_SOI) {
        return embed_jpeg_icc(image, icc);
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "only PNG and JPEG images are supported",
    ))
}

fn embed_png_icc(image: &[u8], icc: &[u8]) -> io::Result<Vec<u8>> {
    let chunks = read_png_chunks(image)?;
    let ihdr = chunks
        .first()
        .filter(|chunk| chunk.name == *b"IHDR")
        .ok_or_else(|| invalid_data("missing PNG IHDR chunk"))?;

    let mut output = Vec::with_capacity(image.len() + icc.len());
    output.extend_from_slice(PNG_SIGNATURE);
    write_png_chunk(&mut output, &ihdr.name, ihdr.data)?;
    write_png_chunk(&mut output, b"iCCP", &png_iccp_data(icc)?)?;

    for chunk in chunks.iter().skip(1) {
        if chunk.name != *b"iCCP" {
            write_png_chunk(&mut output, &chunk.name, chunk.data)?;
        }
    }

    Ok(output)
}

fn embed_jpeg_icc(image: &[u8], icc: &[u8]) -> io::Result<Vec<u8>> {
    if icc.is_empty() {
        return Err(invalid_input("ICC profile is empty"));
    }

    let icc_segments = jpeg_icc_segments(icc)?;
    let mut output =
        Vec::with_capacity(image.len() + icc_segments.iter().map(Vec::len).sum::<usize>());
    if !image.starts_with(JPEG_SOI) {
        return Err(invalid_data("invalid JPEG SOI marker"));
    }

    output.extend_from_slice(JPEG_SOI);
    let mut offset = 2;
    while offset + 4 <= image.len() && image[offset] == 0xff {
        let marker_start = offset;
        while offset < image.len() && image[offset] == 0xff {
            offset += 1;
        }

        let marker = *image
            .get(offset)
            .ok_or_else(|| invalid_data("truncated JPEG marker"))?;

        if marker == 0xda || marker == 0xd9 || !is_jpeg_metadata_marker(marker) {
            for segment in icc_segments {
                output.extend_from_slice(&segment);
            }
            output.extend_from_slice(&image[marker_start..]);
            return Ok(output);
        }

        let length_offset = offset + 1;
        if length_offset + 2 > image.len() {
            return Err(invalid_data("truncated JPEG segment length"));
        }

        let length = u16::from_be_bytes(image[length_offset..length_offset + 2].try_into().unwrap())
            as usize;
        if length < 2 || length_offset + length > image.len() {
            return Err(invalid_data("invalid JPEG segment length"));
        }

        let next_offset = length_offset + length;
        let payload = &image[length_offset + 2..next_offset];
        if !(marker == 0xe2 && payload.starts_with(ICC_JPEG_MARKER)) {
            output.extend_from_slice(&image[marker_start..next_offset]);
        }

        offset = length_offset + length;
    }

    for segment in icc_segments {
        output.extend_from_slice(&segment);
    }
    output.extend_from_slice(&image[offset..]);
    Ok(output)
}

#[derive(Clone, Copy)]
struct PngChunk<'a> {
    name: [u8; 4],
    data: &'a [u8],
}

fn read_png_chunks(image: &[u8]) -> io::Result<Vec<PngChunk<'_>>> {
    if !image.starts_with(PNG_SIGNATURE) {
        return Err(invalid_data("invalid PNG signature"));
    }

    let mut chunks = Vec::new();
    let mut offset = PNG_SIGNATURE.len();

    while offset + 12 <= image.len() {
        let length = u32::from_be_bytes(image[offset..offset + 4].try_into().unwrap()) as usize;
        let name_offset = offset + 4;
        let data_offset = name_offset + 4;
        let crc_offset = data_offset + length;
        let next_offset = crc_offset + 4;

        if next_offset > image.len() {
            return Err(invalid_data("truncated PNG chunk"));
        }

        let mut name = [0; 4];
        name.copy_from_slice(&image[name_offset..data_offset]);
        chunks.push(PngChunk {
            name,
            data: &image[data_offset..crc_offset],
        });

        offset = next_offset;
        if name == *b"IEND" {
            return Ok(chunks);
        }
    }

    Err(invalid_data("missing PNG IEND chunk"))
}

fn png_iccp_data(icc: &[u8]) -> io::Result<Vec<u8>> {
    if icc.is_empty() {
        return Err(invalid_input("ICC profile is empty"));
    }

    let mut compressed = ZlibEncoder::new(Vec::new(), Compression::default());
    compressed.write_all(icc)?;

    let mut data = Vec::with_capacity("ICC Profile".len() + 2 + icc.len());
    data.extend_from_slice(b"ICC Profile");
    data.push(0);
    data.push(0);
    data.extend_from_slice(&compressed.finish()?);
    Ok(data)
}

fn write_png_chunk(output: &mut Vec<u8>, name: &[u8; 4], data: &[u8]) -> io::Result<()> {
    let length = u32::try_from(data.len()).map_err(|_| invalid_input("PNG chunk is too large"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(name);
    output.extend_from_slice(data);

    let mut hasher = Hasher::new();
    hasher.update(name);
    hasher.update(data);
    output.extend_from_slice(&hasher.finalize().to_be_bytes());
    Ok(())
}

fn jpeg_icc_segments(icc: &[u8]) -> io::Result<Vec<Vec<u8>>> {
    let count = icc.len().div_ceil(JPEG_ICC_PAYLOAD_MAX);
    if count > u8::MAX as usize {
        return Err(invalid_input(
            "ICC profile is too large for JPEG APP2 segments",
        ));
    }

    icc.chunks(JPEG_ICC_PAYLOAD_MAX)
        .enumerate()
        .map(|(index, chunk)| {
            let length = ICC_JPEG_MARKER.len() + 2 + chunk.len() + 2;
            let length = u16::try_from(length)
                .map_err(|_| invalid_input("JPEG ICC segment is too large"))?;

            let mut segment = Vec::with_capacity(length as usize + 2);
            segment.extend_from_slice(&[0xff, 0xe2]);
            segment.extend_from_slice(&length.to_be_bytes());
            segment.extend_from_slice(ICC_JPEG_MARKER);
            segment.push(index as u8 + 1);
            segment.push(count as u8);
            segment.extend_from_slice(chunk);
            Ok(segment)
        })
        .collect()
}

fn is_jpeg_metadata_marker(marker: u8) -> bool {
    matches!(marker, 0xe0..=0xef | 0xfe)
}

fn invalid_data(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn invalid_input(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
