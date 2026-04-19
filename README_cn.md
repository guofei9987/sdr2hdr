# sdr2hdr

给 PNG 或 JPEG 图片嵌入内置 ICC 配置文件。

English: [README.md](README.md)

## 安装

```bash
cargo install sdr2hdr
```

## 命令行

使用 type 号选择内置 ICC：

```bash
sdr2hdr <输入图片> [type] [输出图片]
```

示例：

```bash
sdr2hdr image.png
sdr2hdr image.png 1
sdr2hdr image.png 2 image_hdr.png
```

不传 `type` 时默认使用 `1`。

不传输出路径时，会在原文件名后追加 `_hdr`：

```text
image.png -> image_hdr.png
```

## 代码调用

使用 type 号：

```rust
sdr2hdr::embed_icc_file_with_type("image.png", 1, "image_hdr.png")?;
```

直接使用内置 ICC 字节：

```rust
let output = sdr2hdr::embed_icc(&image_bytes, sdr2hdr::icc::icc1())?;
```

## Type

```text
1  icc1.icc
2  icc2.icc
```

## 仓库示例

仓库中包含示例代码和开发用图片：

```text
example/embed_icc.rs
assets/images/original.png
assets/images/original_hdr.png
```

这些文件不会被打进 crates.io 发布包。

运行仓库示例：

```bash
cargo run --example embed_icc
```

## 测试

```bash
cargo test
```
