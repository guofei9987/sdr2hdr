# sdr2hdr

给 PNG 或 JPEG 图片嵌入 ICC 配置文件。

## 使用

使用外部 ICC 文件：

```bash
cargo run -- <输入图片> <ICC文件> [输出图片]
```

示例：

```bash
cargo run -- test/fixtures/images/original.png src/icc/icc1.icc
cargo run -- test/fixtures/images/original.png src/icc/icc1.icc test/fixtures/images/original_hdr.png
```

不传输出路径时，会在原文件名后追加 `_hdr`：

```text
test/fixtures/images/original.png -> test/fixtures/images/original_hdr.png
```

## 代码调用

使用编译进代码的 ICC：

```rust
let output = sdr2hdr::embed_icc(&image_bytes, sdr2hdr::icc::icc1())?;
```

使用外部 ICC 文件：

```rust
sdr2hdr::embed_icc_file(
    "test/fixtures/images/original.png",
    "src/icc/icc1.icc",
    "test/fixtures/images/original_hdr.png",
)?;
```

## 测试

```bash
cargo test
```
