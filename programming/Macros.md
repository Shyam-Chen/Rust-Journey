# 巨集 (Macros)

```rs
macro_rules! hello_world {
    () => {
        println!("Hello, World!");
    };
}

fn main() {
    hello_world!();
    // Hello, World!
}
```

## 帶參數 (With Parameters)

### 表達式: `$val:expr`

```rs
macro_rules! hello {
    ($text:expr) => {
        println!("Hello, {}!", $text);
    };
}

fn main() {
    hello!("World");
    // Hello, World!
}
```

```rs
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr), +) => {{
        let mut max_val = $x;

        $(
            if $y > max_val {
                max_val = $y;
            }
        )*

        max_val
    }};
}

fn main() {
    let result = max!(1, 3, 2, 5, 4);
    println!("{result}"); // 5
}
```

對函式做封裝：

```rs
fn hello(name: &str) {
    println!("Hello, {name}!");
}

macro_rules! hello {
    () => {
        hello("World")
    }; // 無參數 → 預設值
    ($name:expr) => {
        hello($name)
    }; // 有參數 → 帶入值
}

fn main() {
    hello!(); // Hello, World!
    hello!("Alice"); // Hello, Alice!
}
```

## 內建

### `todo!`

```rs
fn foo() {
    todo!();
}

fn bar() {
    todo!("This function needs to be implemented!");
}

fn main() {
    foo();
    bar();
}
```

### `file!`

```rs
fn main() {
    println!("File: {}", file!());
    // File: src/main.rs
}
```

### `cfg!`

```rs
fn main() {
    if cfg!(unix) {
        println!("Running on Unix!");
    } else {
        println!("Not running on Unix!");
    }
}
```

```rs
fn main() {
   if cfg!(target_os = "linux") {
       println!("Running on Linux!");
   } else {
       println!("Not running on Linux!");
   }
}
```
