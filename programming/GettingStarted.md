# 起手式 (Getting Started)

## 環境設定

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

## 初始化專案

```sh
# 建立執行檔專案 (binary)
$ cargo new <PROJECT_NAME>
# e.g.
$ cargo new my-project
```

### 執行專案程式碼

```sh
$ cargo run
```

正式預覽：

```sh
$ cargo run --release
```

### 編譯專案程式碼

```sh
$ cargo build --release
```

直接執行編譯後的二進位檔：

```sh
# Windows
>_ .\target\release\<PROJECT_NAME>.exe
# e.g.
>_ .\target\release\my-project.exe

# macOS
$ ./target/release/<PROJECT_NAME>
# e.g.
$ ./target/release/my-project
```

### 為特定專案指定版本

```sh
$ cd <PROJECT_NAME>
$ rustup override set <VERSION_NUMBER>
```
