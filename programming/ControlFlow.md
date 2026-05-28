# 控制流程 (Control Flow)

## 條件判斷 (If/Else)

```rs
fn main() {
    let score = 85;

    if score >= 60 {
        println!("Pass");
    } else {
        println!("Fail");
    }
}
// Pass
```

```rs
fn main() {
    let number = -7;

    if number > 0 {
        println!("Positive");
    } else if number == 0 {
        println!("Zero");
    } else {
        println!("Negative");
    }
}
// Negative
```

```rs
fn main() {
    let age = 20;
    let can_vote = if age >= 18 { "Yes" } else { "No" };
    println!("Can vote? {can_vote}");
    // Can vote? Yes
}
```

## 模式匹配 (Match)

```rs
fn main() {
    let number = 2;

    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他"), // _ 是萬用匹配符
    }
}
// 二
```

匹配多個值，使用 `|`：

```rs
fn main() {
    let number = 2;

    match number {
        1 | 2 => println!("一"),
        3 | 4 | 5 => println!("二"),
        _ => println!("其他"),
    }
}
// 一
```

匹配範圍，使用 `..=`：

```rs
fn main() {
    let number = 2;

    match number {
        1..=10 => println!("一"), // 匹配 1 ~ 10
        11..=20 => println!("二"), // 匹配 11 ~ 20
        _ => println!("其他"),
    }
}
// 一
```

條件守衛，搭配 `if`:

```rs
fn main() {
    let number = 7;

    match number {
        n if n % 2 == 0 => println!("偶數: {n}"),
        n if n % 2 == 1 => println!("奇數: {n}"),
        _ => println!("未知"),
    }
}
// 奇數: 7
```

匹配範圍與綁定值，使用 `@`：

```rs
fn main() {
    let number = 2;

    match number {
        n @ 1..=10 => println!("位於 1 ~ 10 區間: {n}"),
        n @ 11..=20 => println!("位於 11 ~ 20 區間: {n}"),
        _ => println!("其他"),
    }
}
// 位於 1 ~ 10 區間: 2
```

多行區塊：

```rs
fn main() {
    let number = 2;

    match number {
        1 => println!("一"),
        2 => {
            println!("二");
            println!("二");
        } // 無逗號 `,`
        _ => println!("其他"),
    }
}
// 二
// 二
```

多行區塊回傳值：

```rs
fn main() {
    let number = 2;

    let result = match number {
        1 => {
            let a = 10;
            let b = 20;
            a + b
        }
        2 => {
            let multiple = 5;
            number * multiple
        }
        _ => {
            println!("其他");
            0
        }
    };

    println!("result = {result}");
    // result = 10
}
```

## 無限迴圈 (Loops)

```rs
fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count == 2 {
            continue; // 跳過當前計數為 2
        }

        println!("Count = {count}"); // 當前計數

        if count == 3 {
            break; // 跳出 loop
        }
    }

    println!("Loop 結束");
}
// Count = 1
// Count = 3
// Loop 結束
```

回傳值:

```rs
fn main() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // 回傳值
        }
    };

    println!("結果是: {result}");
}
// 結果是: 20
```

## 條件式迴圈 (While Loops)

```rs
fn main() {
    let mut count = 3;

    while count != 0 {
        println!("{count}");
        count -= 1;
    }

    println!("結束");
}
// 3
// 2
// 1
// 結束
```

## 迭代器迴圈 (For Loops)

```rs
fn main() {
    for number in 1..4 {
        println!("{number}");
    }

    println!("結束");
}
// 1
// 2
// 3
// 結束
```

包含:

```rs
fn main() {
    for number in 1..=3 {
        println!("{number}");
    }

    println!("結束");
}
// 1
// 2
// 3
// 結束
```

## `if let`

```rs
fn main() {
    let my_val = Some(7);

    if let Some(val) = my_val {
        println!("有找到值: {val}");
    } else {
        println!("沒找到值");
    }
}
// 有找到值: 7
```

相當於:

```rs
fn main() {
    let my_val = Some(7);

    match my_val {
        Some(val) => println!("有找到值: {val}"),
        None => println!("沒找到值"),
    }
}
// 有找到值: 7
```

省略 `else` 分支:

```rs
fn main() {
    let my_val = Some(7);

    if let Some(val) = my_val {
        println!("有找到值: {val}");
    }
    // 不用處理 None，直接省略了 else
}
// 有找到值: 7
```

```rs
fn main() {
    let my_val = Some(7);

    if let Some(val) = my_val
        && val > 0
    {
        println!("有找到值且大於零: {val}");
    }
}
// 有找到值且大於零: 7
```

用於:

- 解構 `Option<T>` 的值
- 簡化 `Result<T, E>` 的條件處理
- 避免使用過長的 `match` 語句

## `while let`

```rs
fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("彈出元素: {top}");
    }
}
// 彈出元素: 3
// 彈出元素: 2
// 彈出元素: 1
```

相當於:

```rs
fn main() {
    let mut stack = vec![1, 2, 3];

    loop {
        match stack.pop() {
            Some(top) => println!("彈出元素: {top}"),
            None => break,
        }
    }
}
// 彈出元素: 3
// 彈出元素: 2
// 彈出元素: 1
```
