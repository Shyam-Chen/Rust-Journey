# 作用域 (Scopes)

## 所有權 (Ownership)

```rs
{
    let s = String::from("Hello, World!"); // s 是該 String 的擁有者
    println!("{s}"); // 這裡可以使用 s，因為 s 是該值的擁有者
} // 當 s 離開作用域，其記憶體被自動釋放
```

### 移動所有權

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // 移動所有權給 s2
    // println!("{s1}, World!"); // ❌ 當所有權被移動後，原變數 s1 就無效了，所以不能再使用它。
    println!("{s2}, World!");
}
```

### 借用 (Borrowing, 引用)

不可變借用 `&T`: 只能讀取，不能修改。

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1; // 借用 s1 的所有權，而不是移動
    println!("{s1}, World!"); // s1 仍然有效
    println!("{s2}, World!"); // 使用借用的 s2
}
```

可變借用 `&mut T`: 可以修改，但同一時間只能有一個可變借用。

```rs
fn add_suffix(text: &mut String) {
    text.push_str(", World!");
}

fn main() {
    let mut msg = String::from("Hello");
    add_suffix(&mut msg); // 將 msg 的可變借用傳遞給函式
    println!("{msg}");
}
```

解引用 `*T` (Dereferencing):

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1; // 借用 s1 的所有權，而不是移動
    println!("{s1}, World!"); // s1 仍然有效
    println!("{s2}, World!"); // 使用借用的 s2
    println!("s2 指向的值: {}", *s2); // s2 指向的值: Hello
    // println!("s2 指向的值: {}", s1 == s2); // ❌ 未實作 String == &String
    println!("{}", s1 == *s2); // true
}
```

```rs
fn add_suffix(text: &mut String) {
    *text += ", World!"; // 不使用 push_str 需解引用後修改值
}

fn main() {
    let mut msg = String::from("Hello");
    add_suffix(&mut msg); // 將 msg 的可變借用傳遞給函式
    println!("{msg}");
}
```

```rs
fn increment(num: &mut i32) {
    *num += 1; // 解引用後修改值
}

fn main() {
    let mut val = 10;
    increment(&mut val);
    println!("{val}");
}
```

引用模式 (解構借用的值):

```rs
fn main() {
    let value = Some(String::from("Hello, World!"));

    match &value {
        Some(text) => println!("借用裡面的值: {text}"),
        None => println!("值為 None"),
    }

    println!("value 還可以被訪問: {value:?}");
}
// 借用裡面的值: Hello, World!
// value 還可以被訪問: Some("Hello, World!")
```

```rS
fn main() {
    let mut value = Some(String::from("Hello"));

    match &mut value {
        Some(text) => text.push_str(", World!"),
        None => println!("值為 None"),
    }

    println!("value 還可以被訪問: {value:?}");
}
// value 還可以被訪問: Some("Hello, World!")
```

動態派發 `&dyn`:

建立特徵的動態引用。

```rs
use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn print_area(shape: &dyn Shape) {
    println!("面積: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let shape: &dyn Shape = &circle; // 借用一個 Circle 型別，作為 dyn Shape 使用
    print_area(shape); // 傳遞引用給函式
    // 面積: 12.566370614359172
}
```

改爲使用泛型為靜態派發 (編譯期):

```rs
fn print_area<T: Shape>(shape: &T) {
    println!("面積: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2.0 };
    print_area(&circle);
    // 面積: 12.566370614359172
}
```

### 複製 (克隆, Clone)

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // 複製所有權
    println!("{s1}, World!");
    println!("{s2}, World!");
}
```

### 移動 (Move)

```rs
fn main() {
    let s = String::from("Hello");
    let closure = || println!("{s}"); // 閉包只讀取外部變數 s
    closure(); // 使用閉包
    println!("{s}"); // s 仍然有效
}
```

移轉:

```rs
fn main() {
    let s = String::from("Hello");
    let closure = move || println!("{s}"); // 閉包只讀取外部變數 s
    closure(); // 使用閉包
    // println!("{s}"); // ❌ 已被移轉，所以無效
}
```

#### 自動移轉給函式

```rs
fn push_four(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.push(4);
    numbers
}

fn main() {
    let original = vec![1, 2, 3];
    let updated = push_four(original);
    // println!("{original:?}"); // ❌ original 已被移轉，不能再使用
    println!("{updated:?}"); // [1, 2, 3, 4]
}
```

#### 複製 (拷貝, Copy) 型別的特徵

```rs
fn main() {
    let val = 7;
    let closure = || println!("{val}"); // 閉包只讀取外部變數 val
    closure(); // 使用閉包
    println!("{val}"); // val 仍然有效
}
```

移轉為直接複製值:

```rs
fn main() {
    let val = 7;
    let closure = move || println!("{val}");
    closure();
    println!("{val}"); // val 仍然有效
}
```

## 資源獲取即初始化 (RAII)

```rs
{
    let file = std::fs::File::open("data.txt").unwrap();
    // 使用 file ...
} // file 離開作用域，自動關閉文件
```
