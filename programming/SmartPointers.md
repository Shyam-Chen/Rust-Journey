# 智慧指標 (Smart Pointers)

## 堆積分配 (Heap Allocation) `Box<T>`

Rust 預設所有變數都分配在堆疊 (stack) 上。

但有些情況下我們需要在堆積 (heap) 分配記憶體，例如:

- 編譯期無法確定大小的資料結構 (如遞迴型別)
- 需要移動大型資料而不想複製內容
- 想要明確控制所有權或生命週期
  - 擁有唯一所有權
  - 離開範疇時自動釋放記憶體

這時候就會用到 `Box<T>`。

```rs
fn main() {
    let boxed = Box::new(7);
    println!("{boxed}");
    // 7
}
```

```rs
fn main() {
    let mut boxed = Box::new(7);
    *boxed += 3;
    println!("{boxed}");
    // 10
}
```

```rs
fn add_boxed(x: &mut Box<i32>, y: i32) {
    // 兩層解引用 (Dereference): 第一層 &mut Box<i32> -> Box<i32>，第二層 Box -> i32
    **x += y;
}

fn main() {
    let mut boxed = Box::new(7);
    add_boxed(&mut boxed, 3);
    println!("{boxed}");
    // 10
}
```

## 參考計數 (Reference Counted) `Rc<T>`

多重所有權 + 共享相同資料

並行 (Concurrency) 使用改 `Arc<T>`。

## 可變借用檢查 `RefCell<T>`

並行 (Concurrency) 使用改 `Mutex<T>`。
