# 並行 (Parallelism)

```rs
use std::thread;

fn main() {
    println!("主執行緒 - 1");

    let handle = thread::spawn(|| {
        println!("來自另一個執行緒的問候！");
    });

    println!("主執行緒 - 2");

    handle.join().unwrap();

    println!("主執行緒 - 3");
}
```

## 共享資料

```rs
use std::sync::Arc;
```

## 互斥鎖

透過互斥訪問來解決共享資料的競爭條件問題。

```rs
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(1);

    {
        // 上鎖並且更新數值
        let mut num = data.lock().unwrap();
        *num = 7;
    }

    println!("Final value: {:?}", data.lock().unwrap());
}
```

```rs
use std::sync::Mutex;

// 建立一個全域變數 COUNTER，包在 Mutex 中
static COUNTER: Mutex<i32> = Mutex::new(0);

fn main() {
    {
        // 鎖定 Mutex，取得可變引用
        let mut num = COUNTER.lock().unwrap();
        *num += 1; // 解引用 (Dereference)，修改裡面的值
        println!("目前計數：{}", *num);
        // 目前計數：1
    }

    {
        // 再次鎖定
        let mut num = COUNTER.lock().unwrap();
        *num += 1; // 解引用 (Dereference)，修改裡面的值
        println!("目前計數：{}", *num);
        // 目前計數：2
    }
}
```

## 佇列 (Queue) 與 工作執行緒 (Worker) 模型

生產者 (Producer) 與消費者 (Consumer) 模型

```rs
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    // 建立一個多生產者單消費者的通訊管道
    let (tx, rx) = mpsc::channel();

    // 使用 Mutex 保護訊息消費者
    let rx = Arc::new(Mutex::new(rx));

    // 建立一組 Worker
    let mut workers = vec![];

    for i in 0..4 {
        let rx = Arc::clone(&rx);

        let worker = thread::spawn(move || {
            loop {
                // 從佇列中鎖定
                let job = rx.lock().unwrap().recv();

                // 處理任務
                match job {
                    Ok(task) => {
                        println!("Worker {i} is processing: {task}");
                    }
                    Err(_) => {
                        // 通訊管道已關閉，退出迴圈
                        break;
                    }
                }
            }
        });

        workers.push(worker);
    }

    // 生成一些任務 (生產者角色)
    for task in 0..10 {
        tx.send(task).unwrap();
        println!("Sent task: {task}");
    }

    // 關閉通訊管道，讓工作執行緒知道沒新的任務了
    drop(tx);

    // 等待所有執行緒完成
    for worker in workers {
        worker.join().unwrap();
    }
}
```
