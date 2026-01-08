# 模組 (Modules)

```rs
// src/math.rs
#![allow(dead_code)]

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> i32 {
    a / b
}
```

```rs
// src/main.rs
mod math; // 引入

use math::{add, sub};

fn main() {
    let val = add(1, 6);
    println!("1 + 6 = {val}");
    // 1 + 6 = 7

    let val = sub(1, 6);
    println!("1 - 6 = {val}");
    // 1 - 6 = -5
}
```

`path` 屬性:

將 `src/math.rs` 改成 `src/elementary_arithmetic.rs`

```rs
#[path = "./elementary_arithmetic.rs"]
mod math;

use math::{add, sub};
```

## 預導模組 (Prelude)

```rs
// my_prelude.rs
pub use std::collections::HashMap;
pub use std::sync::Arc;
```

```rs
// main.rs
mod my_prelude;

use my_prelude::*;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("key".to_string(), 42);
    println!("{map:?}");
}
```

## 模組樹

```sh
$ cargo install cargo-modules --locked
```

## 工作空間 (Workspaces)

### `pub(crate)`

### `pub(super)`

### `pub(in path)`
