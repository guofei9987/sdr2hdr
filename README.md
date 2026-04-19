# sdr2hdr

给 PNG 或 JPEG 图片嵌入 ICC 配置文件。

## 使用

使用外部 ICC 文件：

```bash
cargo run -- <输入图片> <ICC文件> [输出图片]
```

示例：

```bash
cargo run -- assets/images/original.png src/icc/icc1.icc
cargo run -- assets/images/original.png src/icc/icc1.icc assets/images/original_hdr.png
```

不传输出路径时，会在原文件名后追加 `_hdr`：

```text
assets/images/original.png -> assets/images/original_hdr.png
```

## 代码调用

使用编译进代码的 ICC：

```rust
let output = sdr2hdr::embed_icc(&image_bytes, sdr2hdr::icc::icc1())?;
```

使用外部 ICC 文件：

```rust
sdr2hdr::embed_icc_file(
    "assets/images/original.png",
    "src/icc/icc1.icc",
    "assets/images/original_hdr.png",
)?;
```

## 示例

```text
example/embed_icc.rs          给用户参考的示例代码
assets/images/original.png    示例输入图
assets/images/original_hdr.png 已嵌入 ICC 的示例输出图
```

运行示例代码：

```bash
cargo run --example embed_icc
```

## 目录

```text
src/                 源码
src/icc/             编译进代码的 ICC 文件
example/             给用户参考的示例代码
assets/images/       示例和测试共用的图片
test/                集成测试
```

## 测试

```bash
cargo test
```
