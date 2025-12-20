# 2215. 找出兩個陣列的差異 (Find the Difference of Two Arrays)

給定兩個**索引從 `0` 開始**的整數陣列 `nums1` 和 `nums2`，回傳一個長度為 `2` 的列表 `answer`，其中：

- `answer[0]` 是 `nums1` 中所有**不**存在於 `nums2` 中的不同整數组成的列表。
- `answer[1]` 是 `nums2` 中所有**不**存在於 `nums1` 中的不同整數组成的列表。

**注意**，列表中的整數可以按**任意**順序回傳。

範例 1：

```coffee
輸入: nums1 = [1,2,3], nums2 = [2,4,6]
輸出: [[1,3],[4,6]]
說明:
對 nums1 來說，nums1[1] = 2 存在於 nums2 的索引 0 處，而 nums1[0] = 1 和 nums1[2] = 3 不存在於 nums2 中。因此，answer[0] = [1,3]。
對 nums2 來說，nums2[0] = 2 存在於 nums1 的索引 1 處，而 nums2[1] = 4 和 nums2[2] = 6 不存在於 nums1 中。因此，answer[1] = [4,6]。
```

範例 2：

```coffee
輸入: nums1 = [1,2,3,3], nums2 = [1,1,2,2]
輸出: [[3],[]]
說明:
對 nums1 來說，nums1[2] 和 nums1[3] 不存在於 nums2 中。由於 nums1[2] == nums1[3]，它們的值僅包含一次，並且 answer[0] = [3]。
nums2 中的每個整數都存在於 nums1 中。因此，answer[1]=[]。
```

## 解題

```rs
use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        // 將 nums1 轉成 HashSet，自動去除重複元素
        let set1: HashSet<_> = nums1.into_iter().collect();

        // 將 nums2 轉成 HashSet，自動去除重複元素
        let set2: HashSet<_> = nums2.into_iter().collect();

        // 找出只在 set1，不在 set2 的元素
        let diff1: Vec<_> = set1.difference(&set2).cloned().collect();

        // 找出只在 set2，不在 set1 的元素
        let diff2: Vec<_> = set2.difference(&set1).cloned().collect();

        // 回傳結果
        vec![diff1, diff2]
    }
}
```
