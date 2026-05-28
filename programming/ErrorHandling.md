# 錯誤處理 (Error Handling)

在 Rust 裡的兩種錯誤處理風格:

- 不可復原的錯誤: `panic!()`
- 可復原的錯誤: `Result<T, E>`

然而，當程式有很多層函式、每層都要回傳錯誤時，都會遇到型別轉換問題。

透過 `anyhow` 讓錯誤型別變成「通用的」，並且保留錯誤追蹤 (backtrace)。

```sh
$ cargo add anyhow
```

```rs
use anyhow::{Result, anyhow};

fn might_fail(x: i32) -> Result<i32> {
    if x == 0 {
        Err(anyhow!("x cannot be zero"))
    } else {
        Ok(x)
    }
}

fn main() -> Result<()> {
    let val = might_fail(0)?; // ? 運算子可以直接傳遞任何錯誤 (不需轉型)
    println!("{val}");
    Ok(())
}
// ❌ Error: x cannot be zero
```
