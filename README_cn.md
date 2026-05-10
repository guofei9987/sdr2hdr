# sdr2hdr

使用 HDR 技术让 PNG 或 JPEG 图片获得高亮显示效果。

English: [README.md](README.md)

## 在线试用

[https://www.guofei.site/os/sdr2hdr.html](https://www.guofei.site/os/sdr2hdr.html)

## 安装

```bash
cargo install sdr2hdr
```

## 命令行

```bash
sdr2hdr <输入图片> [模式] [输出图片]
```

示例：

```bash
sdr2hdr image.png
sdr2hdr image.png 1
sdr2hdr image.png 2 image_hdr.png
```

不传 `模式` 时默认使用 `1`。

不传输出路径时，会自动在文件名后追加 `_hdr`：

```text
image.png -> image_hdr.png
```

## 代码调用

使用内置 HDR 模式：

```rust
sdr2hdr::embed_icc_file_with_type("image.png", 1, "image_hdr.png")?;
```

直接处理图片字节：

```rust
let output = sdr2hdr::embed_icc(&image_bytes, sdr2hdr::icc::icc1())?;
```

## 模式

```text
1  HDR 模式 1
2  HDR 模式 2
```

## 仓库示例

仓库中包含示例代码和测试图片：

```text
example/embed_icc.rs
assets/images/original.png
assets/images/original_hdr.png
```

这些文件不会被打包进 crates.io。

运行示例：

```bash
cargo run --example embed_icc
```

## 测试

```bash
cargo test
```