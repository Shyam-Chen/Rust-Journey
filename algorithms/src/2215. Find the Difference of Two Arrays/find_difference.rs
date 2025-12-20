struct Solution;

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

#[cfg(test)]
#[path = "./find_difference_test.rs"]
mod tests;
