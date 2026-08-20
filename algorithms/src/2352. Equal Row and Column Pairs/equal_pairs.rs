struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        // rows 用來記錄每一種 row 出現幾次
        //
        // 例如：
        // [3, 2, 1] 出現 2 次
        // [1, 7, 6] 出現 1 次
        let mut row_count: HashMap<Vec<i32>, i32> = HashMap::new();

        for row in &grid {
            // row 是 &[i32]，轉成 Vec<i32> 作為 HashMap 的 key
            *row_count.entry(row.clone()).or_insert(0) += 1;
        }

        let mut answer = 0;

        // 逐一處理每一個 column
        for col_index in 0..n {
            let mut column = Vec::with_capacity(n);

            for row_index in 0..n {
                column.push(grid[row_index][col_index]);
            }

            // 如果這個 column 也出現在 row_count 中，
            // 就代表它可以和相同的 row 配對。
            //
            // 若某個 row 出現多次，就要加上出現次數。
            if let Some(&count) = row_count.get(&column) {
                answer += count;
            }
        }

        answer
    }
}

#[cfg(test)]
#[path = "./equal_pairs_test.rs"]
mod tests;
