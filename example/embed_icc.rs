use std::io;

fn main() -> io::Result<()> {
    sdr2hdr::embed_icc_file_with_type(
        "assets/images/original.png",
        1,
        "assets/images/original_hdr.png",
    )
}
