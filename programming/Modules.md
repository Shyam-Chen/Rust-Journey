# 模組 (Modules)

- `pub`：公開
  - `pub(crate)`：只限本 crate 可見
  - `pub(super)`：只對父模組可見
  - `pub(in path)`：只對指定路徑可見
- `mod`：宣告模組
- `use`：引入
  - `pub use`：重新導出
- `self::`：當前模組路徑
- `super::`：父模組路徑，類似檔案路徑的 `../`
- `crate::`：根模組絕對路徑
- `as`：重新命名 (別名，Alias)

```rs
use std::collections::HashMap as Map;
```

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
mod math;

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

## `self::`

```rs
// src/hello.rs
pub fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn run() {
    self::hello("Alice");
}
```

從相對路徑：

```rs
mod hello;

fn main() {
    hello::run();
    // Hello, Alice!
}
```

或是從 `crate::` 根開始的絕對路徑：

```rs
mod hello;

fn main() {
    crate::hello::run();
    // Hello, Alice!
}
```

或是一樣用 `self::` 當前模組：

```rs
mod hello;

fn main() {
    self::hello::run();
    // Hello, Alice!
}
```

或是在頂層使用 `use`：

```rs
mod hello;

use hello::run;

fn main() {
    run();
}
```

或是在函式內局部使用 `use`：

```rs
mod hello;

fn main() {
    use hello::run; // 僅在 main 函式內生效
    run();
}
```

## `super::`

```coffee
src/
├── main.rs
├── parent.rs     ← parent 模組本體
└── parent/
    └── child.rs  ← child 模組本體
```

```rs
// src/main.rs
mod parent;

fn main() {
    parent::greet();
}
```

```rs
// src/parent.rs
mod child;

pub fn greet() {
    println!("Hello from parent");
}
```

```rs
// src/parent/child.rs
pub fn run() {
    super::greet();
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
