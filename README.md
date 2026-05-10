# sdr2hdr

Make PNG or JPEG images appear brighter using HDR techniques.

中文文档: [README_cn.md](README_cn.md)

## Try it online

[https://www.guofei.site/os/sdr2hdr.html](https://www.guofei.site/os/sdr2hdr.html)

## Install

```bash
cargo install sdr2hdr
```

## CLI

```bash
sdr2hdr <input-image> [mode] [output-image]
```

Examples:

```bash
sdr2hdr image.png
sdr2hdr image.png 1
sdr2hdr image.png 2 image_hdr.png
```

If `mode` is omitted, `1` is used.

If `output-image` is omitted, `_hdr` is appended before the extension:

```text
image.png -> image_hdr.png
```

## Library

Use a built-in HDR mode:

```rust
sdr2hdr::embed_icc_file_with_type("image.png", 1, "image_hdr.png")?;
```

Process image bytes directly:

```rust
let output = sdr2hdr::embed_icc(&image_bytes, sdr2hdr::icc::icc1())?;
```

## Modes

```text
1  HDR Mode 1
2  HDR Mode 2
```

## Repository Examples

The repository includes example code and sample images:

```text
example/embed_icc.rs
assets/images/original.png
assets/images/original_hdr.png
```

These files are not included in the crates.io package.

Run the example:

```bash
cargo run --example embed_icc
```

## Test

```bash
cargo test
```