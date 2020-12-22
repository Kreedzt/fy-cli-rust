# fy-cli-rust

使用 Rust 对接有道翻译 API 的翻译项目

## 安装

- Rust 开发者可通过 cargo 安装:
```sh
cargo install fy-cli-rust
```

- 通过 [release](https://github.com/Kreedzt/fy-cli-rust/releases/ "Release") 页面选择二进制下载

## 使用

1. 通过 `--config` 或 `-c` 设置应用ID与密钥

```sh
fy -c key secure
```

2. 直接输入单词翻译

```sh
fy apple
```

## 构建

```sh
cargo build --release
```

## LICENSE
- MIT
