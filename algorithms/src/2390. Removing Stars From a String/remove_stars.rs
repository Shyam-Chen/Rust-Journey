struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        // 使用 Vec 當作 Stack
        let mut stack: Vec<char> = Vec::new();

        // 逐一讀取字串中的字元
        for ch in s.chars() {
            if ch == '*' {
                // 遇到星號，移除左邊最近的一個字元
                //
                // 題目保證每個星號左邊一定有可移除的字元，
                // 因此這裡可以直接呼叫 pop()
                stack.pop();
            } else {
                // 一般字元放入 Stack
                stack.push(ch);
            }
        }

        // 將 Stack 中剩下的字元組合成 String
        stack.into_iter().collect()
    }
}

#[cfg(test)]
#[path = "./remove_stars_test.rs"]
mod tests;
