# 唯一的出現次數 (Unique Number of Occurrences)

給定一個整數陣列 `arr`，如果陣列中每個值的出現次數都是**唯一的**，則回傳 `true`，否則回傳 `false`。

範例 1：

```coffee
輸入: arr = [1,2,2,1,1,3]
輸出: true
說明: 1 出現 3 次，2 出現 2 次，3 出現 1 次。沒有兩個值的出現次數是相同的。
```

範例 2：

```coffee
輸入: arr = [1,2]
輸出: false
```

範例 3：

```coffee
輸入: arr = [-3,0,1,-3,1,1,1,-3,10,0]
輸出: true
```

## 解題

1. 先統計每個數字出現的次數

```coffee
HashMap { 1: 3, 2: 2, 3: 1 }
```

2. 當出現次數有相同的次數時，代表沒有唯一的出現次數，立即回傳 `false`

```coffee
HashSet [3, 2, 1]
```

```rs
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
```
