# 資料型別 (Data Types)

## 數字 (Numbers)

- 整數型別 (Integer Types):
  - 有正負號 (Signed):
    - `i8`: -2<sup>7</sup> ~ 2<sup>7</sup> - 1
    - `i16`: -2<sup>15</sup> ~ 2<sup>15</sup> - 1
    - `i32` (預設): -2<sup>31</sup> ~ 2<sup>31</sup> - 1
    - `i64`: -2<sup>63</sup> ~ 2<sup>63</sup> - 1
    - `i128`: -2<sup>127</sup> ~ 2<sup>127</sup> - 1
    - `isize`: -2<sup>n</sup> ~ 2<sup>n</sup> - 1
      - 在 32 位元系統上，範圍為 -2<sup>31</sup> ~ 2<sup>31</sup> - 1
      - 在 64 位元系統上，範圍為 -2<sup>63</sup> ~ 2<sup>63</sup> - 1
  - 只有正號 (Unsigned):
    - `u8`: 0 ~ 2<sup>8</sup> - 1
    - `u16`: 0 ~ 2<sup>16</sup> - 1
    - `u32`: 0 ~ 2<sup>32</sup> - 1
    - `u64`: 0 ~ 2<sup>64</sup> - 1
    - `u128`: 0 ~ 2<sup>128</sup> - 1
    - `usize`: 0 ~ 2<sup>n</sup> - 1
      - 在 32 位元系統上，範圍為 0 到 2<sup>32</sup> - 1
      - 在 64 位元系統上，範圍為 0 到 2<sup>64</sup> - 1
- 浮點數型別 (Floating-Point Types): `f16`, `f32`, `f64` (預設), `f128`

```rs
fn main() {
    // 自動推斷 (Type Inference)
    let x = 7; // i32
    let y = 7.0; // f64
    println!("{x}, {y}");
    // 7, 7

    // 明確指定 (Explicit Type Annotation)
    let x: i32 = 7;
    let y: f64 = 7.0;
    println!("{x}, {y}");
    // 7, 7

    // 型別後綴 (Type Suffix)
    let x = 7_i32;
    let y = 7_f64;
    println!("{x}, {y}");
    // 7, 7

    // 型別強轉
    let x = 7_i32;
    let y = 7_f64;
    let z = x as f64 + y;
    println!("{z}");
    // 14
}
```

小數點後兩位:

```rs
fn main() {
    let val = 3.14159;

    let n = (val * 100_f64).round() / 100_f64;
    println!("{n}");
    // 3.14

    let s = format!("{val:.2}");
    println!("{s}");
    // 3.14
}
```

可讀性:

```rs
fn main() {
    let val = 1_000_000;
    println!("{val}");
    // 1000000
}
```

## 布林 (Booleans)

```rs
fn main() {
    let disabled = false;
    let has_permission: bool = true;
    println!("{disabled}, {has_permission}");
    // false, true
}
```

## 字元 (Characters)

使用單引號 `'` 表示。

```rs
fn main() {
    let c = 'c';
    println!("{c}");
    // c

    let c: char = 'c';
    println!("{c}");
    // c

    let c = '🦀';
    println!("{c}");
    // 🦀
}
```

## 字串 (Strings)

### 靜態字串, 字串切片 (String Slices) `&str`

https://doc.rust-lang.org/stable/std/primitive.str.html

使用雙引號 `"` 表示。

```rs
fn main() {
    let name = "Alice Smith";
    println!("{name}");
    // Alice Smith

    // [..]
    let name = &name[..];
    println!("{name}");
    // Alice Smith

    // [start..end]
    let first_name = &name[0..5];
    println!("{first_name}");
    // Alice

    // [..end]
    let first_name = &name[..5];
    println!("{first_name}");
    // Alice

    // [start..]
    let last_name = &name[6..];
    println!("{last_name}");
    // Smith

    // [start..=end]
    let last_name = &name[6..=10];
    println!("{last_name}");
    // Smith
}
```

當函式的參數只是給內部函式讀取用時:

```rs
fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let my_name = String::from("Alice");
    greet(&my_name); // Hello, Alice!
    greet("Bob"); // Hello, Bob!
}
```

### 動態字串 (Strings)

https://doc.rust-lang.org/stable/std/string/index.html

```rs
fn main() {
    let text = String::from("Hello, World!");
    println!("{text}");
    // Hello, World!

    let mut text = String::new();
    text.push_str("Hello, World!");
    println!("{text}");
    // Hello, World!

    let text = "Hello, World!".to_string();
    println!("{text}");
    // Hello, World!

    let text: String = "Hello, World!".into(); // 必須明確指定型別
    println!("{text}");
    // Hello, World!

    let world = "World";
    let hello_world = format!("Hello, {world}!");
    println!("{hello_world}");
    // Hello, World!
}
```

附加：

```rs
fn main() {
    let mut text = String::new();
    text.push('A'); // A
    text.push_str("BC"); // ABC
    text += "DE"; // ABCDE
    println!("{text}"); // ABCDE
}
```

## 陣列 (Arrays)

### 靜態陣列 (Arrays)

https://doc.rust-lang.org/stable/std/primitive.array.html

```rs
fn main() {
    let arr = [1, 2, 3, 4, 5]; // 型別推斷為 [i32; 5]
    println!("{arr:?}");
    // [1, 2, 3, 4, 5]

    let zeros = [0; 5];
    println!("{zeros:?}");
    // [0, 0, 0, 0, 0]
}
```

轉成切片:

```rs
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("{slice:?}");
    // [2, 3, 4]
}
```

轉成動態陣列:

```rs
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut vec = arr.to_vec();
    vec.push(6);
    println!("{vec:?}");
    // [1, 2, 3, 4, 5, 6]
}
```

### 動態陣列 (Vectors)

https://doc.rust-lang.org/stable/std/vec/index.html

```rs
fn main() {
    let vec = vec![1, 2, 3];
    println!("{vec:?}");
    // [1, 2, 3]
}
```

#### 迭代器 (Iterators)

```rs
fn main() {
    let vec = vec![1, 2, 3];

    for num in vec {
        println!("{num}");
    }

    // println!("{vec:?}"); // ❌ vec 的所有權在 for 迴圈中被消耗掉 (即已移動)
}
```

引用:

```rs
fn main() {
    let vec = vec![1, 2, 3];

    for num in &vec { // 引用
        println!("{num}");
    }

    println!("{vec:?}");
}
```

```rs
fn main() {
    let vec = vec![1, 2, 3];

    // 引用模式
    for &num in &vec {
        // 沒有 &，因為自動完成了解構
        if num > 0 {
            println!("{num}");
        }
    }

    // 手動解引用
    for num_ref in &vec {
        // 明確解引用
        if *num_ref > 0 {
            println!("{}", *num_ref);
        }
    }
}
```

不可變 `iter`:

```rs
fn main() {
    let vec = vec![1, 2, 3];
    vec.iter().for_each(|num| println!("{num}"));
    println!("{vec:?}");
}
```

可變 `iter_mut`:

```rs
fn main() {
    let mut vec = vec![1, 2, 3];

    vec.iter_mut().for_each(|num| {
        *num += 1;
        println!("{num}");
    });

    println!("{vec:?}");
}
```

`into_iter`，同 `for` 迴圈:

```rs
fn main() {
    let vec = vec![1, 2, 3];
    vec.into_iter().for_each(|num| println!("{num}"));
    // println!("{vec:?}"); // ❌
}
```

索引:

```rs
fn main() {
    let vec = vec![1, 2, 3];

    for (idx, num) in vec.iter().enumerate() {
        println!("索引: {idx}, 數值: {num}");
    }
}
```

#### 排序

```rs
fn main() {
    let mut vec = vec![1, 30, 4, 21, 100000];
    vec.sort();
    println!("{vec:?}");
    // [1, 4, 21, 30, 100000]
}
```

```rs
fn main() {
    let mut vec = vec![1, 30, 4, 21, 100000];
    vec.sort_by(|a, b| b.cmp(a));
    println!("{vec:?}");
    // [100000, 30, 21, 4, 1]
}
```

```rs
fn main() {
    let mut fruits = vec!["Apple", "pear", "Banana", "orange"];
    fruits.sort_by_key(|s| s.to_lowercase());
    println!("{fruits:?}");
    // ["Apple", "Banana", "orange", "pear"]
}
```

## 元組 (Tuples)

https://doc.rust-lang.org/stable/std/primitive.tuple.html

```rs
fn main() {
    let point = (3, 4);
    let x = point.0;  // 訪問第一個元素
    let y = point.1; // 訪問第二個元素
    println!("{x}, {y}");
    // 3, 4
}
```

解構 (Destructuring):

```rs
fn main() {
    let point = (3, 4);
    let (x, y) = point;
    println!("{x}, {y}");
    // 3, 4
}
```

```rs
fn calculate_point() -> ((i32, i32), f64) {
    let (x, y) = (3, 4);
    let distance = ((x * x + y * y) as f64).sqrt();
    ((x, y), distance)
}

fn main() {
    let (point, distance) = calculate_point();
    println!("{:?}, {}", point, distance);
    // (3, 4), 5
}
```

## 結構 (Structures)

```rs
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let mut user = User {
        username: "graydon.hoare".into(),
        email: "graydon.hoare@outlook.com".into(),
        active: true,
    };

    user.email = "graydon.hoare@gmail.com".into();

    println!("username = {}", user.username); // username = graydon.hoare
    println!("email = {}", user.email); // email = graydon.hoare@gmail.com
    println!("active = {}", user.active); // active = true
}
```

### 元組結構 (Tuple Structures)

```rs
struct Color(u8, u8, u8);

fn main() {
    let my_color = Color(128, 64, 255);
    println!("Red: {}", my_color.0); // Red: 128
    println!("Green: {}", my_color.1); // Green: 64
    println!("Blue: {}", my_color.2); // Blue: 255
}
```

## 列舉 (Enumerations)

```rs
#![allow(dead_code)]

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let dir = Direction::Up;

    match dir {
        Direction::Up => println!("Up"),
        Direction::Right => println!("Right"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
    }
}
// Up
```

### 內建列舉

#### `Option<T>`

https://doc.rust-lang.org/std/option/enum.Option.html

```rs
let some_value: Option<i32> = Some(7); // 表示存在值 7
let none_value: Option<i32> = None; // 表示值不存在
```

#### `Result<T, E>`

https://doc.rust-lang.org/std/result/enum.Result.html

```rs
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Ok(val) => println!("Result1: {val}"),
        Err(err) => println!("Error1: {err}"),
    }

    match result2 {
        Ok(val) => println!("Result2: {val}"),
        Err(err) => println!("Error2: {err}"),
    }
}
// Result1: 5
// Error2: Division by zero
```

## 雜湊映射 (Hash Maps)

https://doc.rust-lang.org/std/collections/struct.HashMap.html

```rs
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    map.insert("Alice".into(), 60);
    map.insert("Bob".into(), 70);

    println!("{map:?}");
    // {"Alice": 60, "Bob": 70}
}
```

在 `#[no_std]` 下，需改用 `use hashbrown::HashMap;`，或是需要在性能上壓榨極致也可以用。

轉換 `Vec` 為 `HashMap`:

```rs
use std::collections::HashMap;

fn main() {
    let vec = vec![("Alice", 60), ("Bob", 70), ("Carol", 90)];
    let map: HashMap<_, _> = vec.into_iter().collect();
    println!("{map:?}"); // {"Alice": 60, "Bob": 70, "Carol": 90} (順序未定)
}
```

## 雜湊集合 (Hash Sets)

https://doc.rust-lang.org/std/collections/struct.HashSet.html

```rs
use std::collections::HashSet;

fn main() {
    let mut set: HashSet<&str> = HashSet::new();

    set.insert("apple");
    set.insert("banana");
    set.insert("orange");

    // 重複的元素不會被加入
    set.insert("apple");

    println!("{set:?}");
    // {"apple", "banana", "orange"}
}
```

在 `#[no_std]` 下，需改用 `use hashbrown::HashSet;`，或是需要在性能上壓榨極致也可以用。

轉換 `Vec` 為 `HashSet`:

```rs
use std::collections::HashSet;

fn main() {
    let vec = vec![1, 2, 3, 3, 5];
    let set: HashSet<_> = vec.into_iter().collect();
    println!("{set:?}"); // {1, 2, 3, 5} (順序未定)
}
```

## 型別別名 (Type Alias)

```rs
type Age = u8;

fn print_age(age: Age) {
    println!("年齡是: {age}");
}

fn main() {
    let my_age: Age = 30;
    print_age(my_age);
    // 年齡是: 30
}
```

```rs
type Point = (f64, f64);

fn distance(p1: Point, p2: Point) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let point1: Point = (0.0, 0.0);
    let point2: Point = (3.0, 4.0);
    println!("兩點距離是: {}", distance(point1, point2));
    // 兩點距離是: 5
}
```
