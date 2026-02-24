# 變數 (Variables)

- 使用全小寫和底線分隔

```rs
fn main() {
    let val = 7;
    println!("值為：{val}");
    // 值為：7

    let my_val = 7;
    println!("值為：{my_val}");
    // 值為：7
}
```

可變的 (Mutable):

```rs
fn main() {
    let mut val = 7;
    println!("值為：{val}");
    // 值為：7

    val = 6;
    println!("值為：{val}");
    // 值為：6
}
```

## 常數 (Constants)

- 使用全大寫和底線分隔
- 型別必須明確指定

```rs
fn main() {
    const PI: f64 = 3.14159265;
    println!("{PI}");
    // 3.14159265

    const MAX_POINTS: u32 = 100_000;
    println!("{MAX_POINTS}");
    // 100000
}
```

## 遮蔽 (Shadowing)

```rs
fn main() {
    let val = 7;
    let val = val - 1;

    {
        let val = val * 2;
        println!("在內部範疇的值為：{val}");
        // 在範疇內的值為：12
    }

    println!("值為：{val}");
    // 值為：6
}
```

## 全域變數 (Global Variables)

```rs
static PI: f64 = 3.141592653589793;

fn main() {
    println!("{PI}");
    // 3.141592653589793
}
```

可變的 (Mutable):

- 單執行緒 (Single-thread) 使用 `RefCell<T>`
- 多執行緒 (Multi-thread) 使用 `Mutex<T>`
