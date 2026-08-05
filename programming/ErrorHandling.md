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

```rs
use anyhow::{Context, Result};

fn find_max(numbers: &[i32]) -> Result<&i32> {
    numbers
        .iter()
        .max()
        .context("無法尋找最大值：輸入陣列是空的")
}

fn main() -> Result<()> {
    let numbers = [1, 7, 3, 9, 2];
    let max = find_max(&numbers)?;
    println!("最大值：{max}");
    Ok(())
}
```

當輸入陣列為空時，`max()` 會回傳 `None`。透過 `context()` 將 `None` 轉換成錯誤後，程式會回傳：

```sh
Error: 無法尋找最大值：輸入陣列是空的
```

因此不會因為呼叫 `unwrap()` 而直接觸發 `panic`。

如果使用：

```rs
fn find_max(numbers: &[i32]) -> &i32 {
    numbers.iter().max().unwrap()
}

fn main() {
    let numbers = [1, 7, 3, 9, 2];
    let max = find_max(&numbers);
    println!("最大值：{max}");
}
```

當陣列為空時，`unwrap()` 會嘗試從 `None` 取出值，並造成程式崩潰。`panic` 訊息如下：

```sh
called `Option::unwrap()` on a `None` value
```
