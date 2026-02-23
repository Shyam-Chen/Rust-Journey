# 933. 最近的請求次數 (Number of Recent Calls)

你有個 `RecentCounter` 類別，用來計算特定時間範圍內最近的請求次數。

實現 `RecentCounter` 類別：

- `RecentCounter()` 初始化計數器，最近的請求次數為零。
- `int ping(int t)` 在時間 `t` 新增一個請求，其中 `t` 表示某個時間 (以毫秒為單位)，並回傳過去 `3000` 毫秒內發生的請求次數 (包括新請求)。具體來說，回傳 `[t - 3000, t]` 範圍內發生的請求次數。

**確保**每次 `ping` 呼叫都使用比前一次呼叫嚴格較大的 `t` 值。

範例 1：

```coffee
輸入:
["RecentCounter", "ping", "ping", "ping", "ping"]
[[], [1], [100], [3001], [3002]]
輸出:
[null, 1, 2, 3, 3]
說明:
RecentCounter recentCounter = new RecentCounter();
recentCounter.ping(1);     // requests = [1], 範圍是 [-2999,1], 回傳 1
recentCounter.ping(100);   // requests = [1, 100], 範圍是 [-2900,100], 回傳 2
recentCounter.ping(3001);  // requests = [1, 100, 3001], 範圍是 [1,3001], 回傳 3
recentCounter.ping(3002);  // requests = [1, 100, 3001, 3002], 範圍是 [2,3002], 回傳 3
```

## 解題

```rs
use std::collections::VecDeque;

// 定義 RecentCounter 結構，儲存請求時間的佇列
struct RecentCounter {
    calls: VecDeque<i32>,
}

impl RecentCounter {
    // 初始化計數器
    fn new() -> Self {
        Self {
            calls: VecDeque::new(), // 初始化為空佇列，表示零個請求
        }
    }

    // 處理請求時間，並回傳最近 3000 毫秒內的請求次數
    fn ping(&mut self, t: i32) -> i32 {
        // 將新的請求時間 t 放入佇列尾部
        self.calls.push_back(t);

        // 從佇列頭部移除所有早於 (t - 3000) 的請求，因為它們已不在範圍內
        while let Some(&first_call_time) = self.calls.front() // 檢查佇列頭部的請求時間
            // 若這個請求早於 (t - 3000)，則移除
            && first_call_time < t - 3000
        {
            // 從佇列頭部刪除過期的請求
            self.calls.pop_front();
        }

        // 回傳佇列中剩餘的請求次數 (即最近 3000 毫秒內的請求次數)
        self.calls.len() as i32
    }
}
```
