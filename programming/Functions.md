# 函式 (Functions)

```rs
fn greet() {
    println!("Hello, World!");
}

fn main() {
    greet();
    greet_person("Shyam");
}

fn greet_person(name: &str) {
    println!("Hello, {name}!");
}
```

### 表達式 (Expressions)

```rs
fn main() {
    let val = {
        let a = 5;
        let b = 3;
        a + b // 沒有分號
    };

    println!("值為：{val}");
    // 值為：8
}
```

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b // 沒有分號表示這是表達式，會被當作回傳值
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {result}");
    // 5 + 3 = 8

    let result = sub(5, 3);
    println!("5 - 3 = {result}");
    // 5 - 3 = 2
}
```

### 閉包 (Closures)

```rs
fn main() {
    // 函式定義
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    // 閉包定義
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 }; // Rustfmt 執行後將變為 v4 的樣式
    let add_one_v4 = |x| x + 1;

    let a = add_one_v1(6);
    let b = add_one_v2(6);
    let c = add_one_v3(6);
    let d = add_one_v4(6);

    println!("{a}, {b}, {c}, {d}");
    // 7, 7, 7, 7
}
```

## 實作 (Implementations)

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 初始化，關聯函式（不需要 self）
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // 方法（需要 self）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法（需要 self）
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);

    println!("rect1 面積 = {}", rect1.area());
    // rect1 面積 = 1500
    println!("rect1 可以裝 rect2 嗎？ {}", rect1.can_hold(&rect2));
    // rect1 可以裝 rect2 嗎？ true
}
```

可變的 (Mutable):

```rs
struct Counter {
    number: i32,
}

impl Counter {
    fn new() -> Self {
        Self { number: 0 }
    }

    fn inc(&mut self) {
        self.number += 1;
    }

    fn count(&self) -> i32 {
        self.number
    }
}

fn main() {
    let mut counter = Counter::new();
    counter.inc();
    println!("count = {}", counter.count());
    // count = 1
}
```

## 特徵 (Traits)

用於定義共享的行為。

```rs
trait Animal {
    fn sound(&self);
}

// ---

struct Cat;

impl Animal for Cat {
    fn sound(&self) {
        println!("喵 Meow");
    }
}

// ---

struct Dog;

impl Animal for Dog {
    fn sound(&self) {
        println!("汪 Woof");
    }
}

// ---

fn animal_sound_stack(animal: &dyn Animal) {
    animal.sound();
}

fn animal_sound_heap(animal: Box<dyn Animal>) {
    animal.sound();
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    cat.sound(); // 喵 Meow
    dog.sound(); // 汪 Woof

    animal_sound_stack(&cat); // 喵 Meow
    animal_sound_stack(&dog); // 汪 Woof

    animal_sound_heap(Box::new(cat)); // 喵 Meow
    animal_sound_heap(Box::new(dog)); // 汪 Woof
}
```

`&dyn` 為動態派發，至[範疇 (Scopes)](./Scopes.md) 了解更多。

## 泛型 (Generics)

```rs
use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let int_val = add(1, 6);
    let float_val = add(3.7, 2.1);
    println!("int_val = {int_val}"); // int_val = 7
    println!("float_val = {float_val}"); // float_val = 5.800000000000001
}
```

指定特徵限制:

```rs
use std::ops::Add;

fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

fn main() {
    let int_val = add(1, 6);
    let float_val = add(3.7, 2.1);
    println!("int_val = {int_val}"); // int_val = 7
    println!("float_val = {float_val}"); // float_val = 5.800000000000001
}
```

結構的泛型:

```rs
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = Point { x: 1, y: 6 };
    let float_point = Point { x: 3.7, y: 2.1 };

    let dx = float_point.x - int_point.x as f64;
    let dy = float_point.y - int_point.y as f64;
    println!("Distance: {}", (dx * dx + dy * dy).sqrt());
    // Distance: 4.743416490252569
}
```

結構和實作的泛型:

```rs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let int_point = Point::new(3, 4);
    println!("{}, {}", int_point.x(), int_point.y());
    // 3, 4

    let float_point = Point::new(3.0, 4.0);
    println!("{}, {}", float_point.x(), float_point.y());
    // 3, 4
}
```

指定多個特徵:

```rs
use std::ops::Mul;

fn double_val<T>(val: T) -> T
where
    T: Mul<Output = T> + From<u8>,
{
    val * T::from(2)
}

fn main() {
    let int_val = 10;
    let float_val = 7.2;
    println!("Int doubled: {}", double_val(int_val)); // Int doubled: 20
    println!("Float doubled: {}", double_val(float_val)); // Float doubled: 14.4
}
```
