# 檔案操作 (File Operations)

## 讀取整個文字檔

檔案內容須是 UTF-8 編碼格式。

```rs
use anyhow::Result;
use std::fs;

fn read_package_metadata() -> Result<String> {
    let content = fs::read_to_string("Cargo.toml")?;
    Ok(content)
}

fn main() -> Result<()> {
    let package_metadata = read_package_metadata()?;
    println!("{package_metadata}");
    Ok(())
}
```

### 使用 `File` 讀取

```rs
use anyhow::Result;
use std::fs;
use std::io::Read;

fn read_package_metadata() -> Result<String> {
    let mut file = fs::File::open("Cargo.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<()> {
    let package_metadata = read_package_metadata()?;
    println!("{package_metadata}");
    Ok(())
}
```

當檔案不存在就建立 (`.create(true)`)：

```rs
use anyhow::Result;
use std::fs;
use std::io::Read;

fn read_package_metadata() -> Result<String> {
    let mut file = fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("Cargo.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<()> {
    let package_metadata = read_package_metadata()?;
    println!("{package_metadata}");
    Ok(())
}
```

### 緩衝區：逐行讀取

```rs
use anyhow::Result;
use std::fs;
use std::io::{BufRead, BufReader};

fn read_package_metadata() -> Result<String> {
    let file = fs::File::open("Cargo.toml")?;
    let reader = BufReader::new(file);
    let mut content = String::new();

    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    Ok(content)
}

fn main() -> Result<()> {
    let package_metadata = read_package_metadata()?;
    println!("{package_metadata}");
    Ok(())
}
```

TODO: 待 `fs::File::open_buffered` 功能穩定。

## 拼接路徑

```rs
use anyhow::Result;
use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

fn main() -> Result<()> {
    let workdir = env::current_dir()?;
    println!("{}", workdir.display());

    let mut path = PathBuf::from(workdir);
    path.push("src/main.rs");
    println!("{}", path.canonicalize()?.display());

    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{content}");

    Ok(())
}
```

## 建立與刪除目錄

### 單層目錄

```rs
use std::fs;

fs::create_dir("logs")?;
```

### 多層目錄

```rs
use std::fs;

fs::create_dir_all("logs/2027/jul")?;
```
