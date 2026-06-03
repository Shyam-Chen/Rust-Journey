# 命令列 (Command Line)

## 應用程式 (Application)

```sh
$ cargo new my-cli-app
$ cd my-cli-app
```

```sh
$ cargo add clap -F derive
```

```rs
use clap::Parser;

#[derive(Parser)]
#[command(name = "my-cli-app")]
#[command(about = "我的第一個 CLI 工具", version)]
struct Cli {
    #[arg(short, long, default_value = "World", help = "輸入你的名字")]
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}
```

```sh
$ cargo run
Hello, World!

$ cargo run -- --name Alice
Hello, Alice!

$ cargo run -- -n Bob
Hello, Bob!
```

```sh
$ cargo run -- -h
我的第一個 CLI 工具

Usage: my-cli-app [OPTIONS]

Options:
  -n, --name <NAME>  輸入你的名字 [default: World]
  -h, --help         Print help
  -V, --version      Print version
```

```sh
$ cargo run -- -V
my-cli-app 0.1.0
```

編譯成可執行檔:

```sh
$ cargo build --release
```

將執行檔安裝到本地:

```sh
$ cp ./target/release/my-cli-app $HOME/.cargo/bin/
```

```sh
$ my-cli-app
Hello, World!

$ my-cli-app --name Alice
Hello, Alice!

$ my-cli-app -n Bob
Hello, Bob!
```

## 子命令 (Subcommands)

```rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "my-cli-app")]
#[command(about = "我的第一個 CLI 工具", version)]
struct Cli {
    #[arg(short, long, default_value = "World", help = "輸入你的名字")]
    name: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "安裝指定的 package")]
    Install { package: String },
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);

    if let Some(Commands::Install { package }) = cli.command {
        println!("開始安裝套件: {package}");
    }
}
```

```sh
$ cargo run -- install deno
Hello, World!
開始安裝套件: deno
```

## 表格 (Tables)

```sh
$ cargo add tabled
```

## 進展 (Progresses)

```sh
$ cargo add indicatif
```

## 顏色 (Colors)

```sh
$ cargo add console
```

## 提示字元 (Prompts)

```sh
$ cargo add dialoguer
```

## 非同步 (Asynchronous)

```sh
$ cargo add tokio -F full
```

```rs
#[tokio::main]
async fn main() {
    // ...
}
```

```rs
// ...
    if let Some(Commands::Install { package }) = cli.command {
        handle_install(package).await;
    }
// ...
```

```rs
async fn handle_install(package: String) {
    println!("正在準備安裝套件: {package}");

    // 模擬非同步的操作 (假設下載一個 2 秒完成的任務)
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("套件 {package} 安裝完成!");
}
```

加上旋轉器 (Spinner):

```rs
use indicatif::ProgressBar;
use tokio::time::{Duration, sleep};

async fn handle_install(package: String) {
    // 建立旋轉器
    let spinner = ProgressBar::new_spinner();

    // 設定旋轉器的開始訊息
    spinner.set_message(format!("正在準備安裝套件: {}", package));

    // 開始旋轉
    spinner.enable_steady_tick(Duration::from_millis(200));

    // 模擬非同步的安裝作業
    sleep(Duration::from_secs(2)).await;

    // 停止旋轉，並顯示任務完成訊息
    spinner.finish_with_message(format!("套件 {} 安裝完成!", package));
}
```

## 開箱 (Unboxing Crates)

```sh
$ cargo install bat --locked
```

```sh
$ cargo install oha --locked
```
