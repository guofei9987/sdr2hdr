# sdr2hdr

Embed built-in ICC profiles into PNG or JPEG images.

中文文档: [README_cn.md](README_cn.md)

## Install

```bash
cargo install sdr2hdr
```

## CLI

Use a profile type to select a built-in ICC profile:

```bash
sdr2hdr <input-image> [type] [output-image]
```

Examples:

```bash
sdr2hdr image.png
sdr2hdr image.png 1
sdr2hdr image.png 2 image_hdr.png
```

When `type` is omitted, `1` is used.

When `output-image` is omitted, `_hdr` is appended before the extension:

```text
image.png -> image_hdr.png
```

## Library

Use a profile type:

```rust
sdr2hdr::embed_icc_file_with_type("image.png", 1, "image_hdr.png")?;
```

Use built-in ICC bytes directly:

```rust
let output = sdr2hdr::embed_icc(&image_bytes, sdr2hdr::icc::icc1())?;
```

## Profile Types

```text
1  icc1.icc
2  icc2.icc
```

## Repository Examples

The repository contains example code and sample images for development:

```text
example/embed_icc.rs
assets/images/original.png
assets/images/original_hdr.png
```

These files are not included in the crates.io package.

Run the repository example:

```bash
cargo run --example embed_icc
```

## Test

```bash
cargo test
```
