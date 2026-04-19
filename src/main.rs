use std::env;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 4 {
        eprintln!("usage: sdr2hdr <input.png|jpg|jpeg> [type: 1|2] [output.png|jpg|jpeg]");
        std::process::exit(2);
    }

    let input = Path::new(&args[1]);
    let icc_type = args
        .get(2)
        .map(|value| parse_icc_type(value))
        .transpose()?
        .unwrap_or(1);
    let output = args
        .get(3)
        .map(PathBuf::from)
        .unwrap_or_else(|| default_output_path(input));

    sdr2hdr::embed_icc_file_with_type(input, icc_type, &output)?;
    println!("saved: {}", output.display());
    Ok(())
}

fn parse_icc_type(value: &str) -> io::Result<u8> {
    let icc_type = value
        .parse::<u8>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "ICC type must be a number"))?;

    sdr2hdr::icc::by_type(icc_type)?;
    Ok(icc_type)
}

fn default_output_path(input: &Path) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let ext = input.extension().unwrap_or_default().to_string_lossy();
    input.with_file_name(format!("{stem}_hdr.{ext}"))
}
