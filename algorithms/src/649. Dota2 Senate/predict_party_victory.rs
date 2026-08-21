struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();

        let mut radiant_queue = VecDeque::new();
        let mut dire_queue = VecDeque::new();

        // 將議員的原始索引放入對應的 Queue
        for (index, senator) in senate.chars().enumerate() {
            if senator == 'R' {
                radiant_queue.push_back(index);
            } else if senator == 'D' {
                dire_queue.push_back(index);
            }
        }

        while !radiant_queue.is_empty() && !dire_queue.is_empty() {
            if let (Some(radiant_index), Some(dire_index)) =
                (radiant_queue.pop_front(), dire_queue.pop_front())
            {
                if radiant_index < dire_index {
                    // Radiant 先行動，禁止這位 Dire 投票
                    // Radiant 自己仍然存活，進入下一輪
                    radiant_queue.push_back(radiant_index + n);
                } else {
                    // Dire 先行動，禁止這位 Radiant 投票
                    // Dire 自己仍然存活，進入下一輪
                    dire_queue.push_back(dire_index + n);
                }
            }
        }

        if radiant_queue.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

#[cfg(test)]
#[path = "./predict_party_victory_test.rs"]
mod tests;
