struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // 建立兩個字串的字元頻率 HashMap
        // key: 字元, value: 出現次數
        let mut freq1: HashMap<char, i32> = HashMap::new();
        let mut freq2: HashMap<char, i32> = HashMap::new();

        for c in word1.chars() {
            *freq1.entry(c).or_insert(0) += 1;
        }

        for c in word2.chars() {
            *freq2.entry(c).or_insert(0) += 1;
        }

        // 比較字元種類（Key Set 必須相同）
        // 操作一只能交換位置，操作二只能在「已存在」的字元間互換頻率
        // 若 word1 有 'x' 而 word2 沒有，無論如何操作都無法讓兩者相同
        let keys1: HashSet<char> = freq1.keys().cloned().collect();
        let keys2: HashSet<char> = freq2.keys().cloned().collect();

        if keys1 != keys2 {
            return false;
        }

        // 比較頻率的多重集合（排序後的 Value 必須相同）
        // 操作二可以把頻率「互換」給不同字元
        // 例如：  word1: a=2, b=3  →  可變成 a=3, b=2
        // 所以只需要頻率的集合（排序後）相同即可，不在意哪個字元對應哪個頻率
        let mut vals1: Vec<i32> = freq1.values().cloned().collect();
        let mut vals2: Vec<i32> = freq2.values().cloned().collect();

        vals1.sort();
        vals2.sort();

        vals1 == vals2
    }
}

#[cfg(test)]
#[path = "./close_strings_test.rs"]
mod tests;
