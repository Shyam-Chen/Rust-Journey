struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        // 統計每個數字的出現次數
        let mut counts = HashMap::new();

        for num in arr {
            *counts.entry(num).or_insert(0) += 1;
        }

        // 檢查出現次數是否為唯一的
        let mut seen_counts = HashSet::new();

        for &count in counts.values() {
            // 一旦 insert 失敗，代表這個次數之前已經出現過了
            if !seen_counts.insert(count) {
                return false;
            }
        }

        // 若所有的出現次數都成功插入，代表全部都是唯一的
        true
    }
}

#[cfg(test)]
#[path = "./unique_occurrences_test.rs"]
mod tests;
