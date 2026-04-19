use std::env;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: sdr2hdr <input.png|jpg|jpeg> <icc.bin> [output.png|jpg|jpeg]");
        std::process::exit(2);
    }

    let input = Path::new(&args[1]);
    let icc = Path::new(&args[2]);
    let output = args
        .get(3)
        .map(PathBuf::from)
        .unwrap_or_else(|| default_output_path(input));

    sdr2hdr::embed_icc_file(input, icc, &output)?;
    println!("saved: {}", output.display());
    Ok(())
}

fn default_output_path(input: &Path) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let ext = input.extension().unwrap_or_default().to_string_lossy();
    input.with_file_name(format!("{stem}_hdr.{ext}"))
}
