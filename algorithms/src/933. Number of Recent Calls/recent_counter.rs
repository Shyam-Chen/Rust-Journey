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

#[cfg(test)]
#[path = "./recent_counter_test.rs"]
mod tests;
