# 起手式 (Getting Started)

```sh
# Windows (WinGet)
>_ winget install -e --id Rustlang.Rust.MSVC

# macOS (Homebrew)
$ brew install rust
```

```sh
$ rustc --version
rustc 1.92.0 (ded5c06cf 2025-12-08)

$ cargo --version
cargo 1.92.0 (344c4567c 2025-10-21)
```

更新至最新版：

```sh
$ rustup update
```

### 為特定專案指定版本

```sh
$ cd <PROJECT_NAME>
$ rustup override set <VERSION_NUMBER>
```
