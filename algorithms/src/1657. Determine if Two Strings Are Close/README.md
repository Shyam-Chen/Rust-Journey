# 1657. 確定兩個字串是否相近 (Determine if Two Strings Are Close)

若可以透過以下操作，從其中一個字串得到另一個字串，則這兩個字串被視為**相近**：

- 操作一：交換任意兩個**已存在**的字元。
  - 例如：`ab̲cde̲ -> ae̲cdb̲`
- 操作二：將某個**已存在**字元的**所有**出現位置轉換成另一個**已存在**的字元，並同時將那個字元也轉換回來。
  - 例如：`a̲a̲ca̲b̲b̲ -> b̲b̲cb̲a̲a̲`（所有的 `a` 變成 `b`，所有的 `b` 變成 `a`）

可以對任一字串執行任意次數的操作。

給定兩個字串 `word1` 與 `word2`，若 `word1` 與 `word2` **相近**，則回傳 `true`，否則回傳 `false`。

範例 1：

```coffee
輸入: word1 = "abc", word2 = "bca"
輸出: true
說明: 可以透過 2 次操作從 word1 獲得 word2。
應用操作一: "ab̲c̲" -> "ac̲b̲"
應用操作一: "a̲cb̲" -> "b̲ca̲"
```

範例 2：

```coffee
輸入: word1 = "a", word2 = "aa"
輸出: false
說明: 在任意數量的操作中都不可能從 word1 獲得 word2，反之亦然。
```

範例 3：

```coffee
輸入: word1 = "cabbba", word2 = "abbccc"
輸出: true
說明: 可以透過 3 次操作從 word1 獲得 word2。
應用操作一: "cab̲bba̲" -> "caa̲bbb̲"
應用操作二: "c̲aab̲b̲b̲" -> "b̲aac̲c̲c̲"
應用操作二: "b̲a̲a̲ccc" -> "a̲b̲b̲ccc"
```

## 解題

```rs
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
```
