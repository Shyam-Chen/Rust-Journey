struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        // Stack 中儲存：
        // (進入目前括號前的字串, 目前括號內容要重複的次數)
        let mut stack: Vec<(String, usize)> = Vec::new();

        // 目前正在組合的數字，例如 "123" 會變成 123
        let mut number: usize = 0;

        // 目前正在處理的字串
        let mut current = String::new();

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                // 將字元數字加入 number
                //
                // 例如：
                // 原本 number = 12
                // 遇到 '3' 後：
                // number = 12 * 10 + 3 = 123
                number = number * 10 + (ch as u8 - b'0') as usize;
            } else if ch == '[' {
                // 遇到 '['，代表即將開始處理一層新的括號內容。
                //
                // 先保存：
                // 1. 進入括號前的字串 current
                // 2. 這層括號要重複的次數 number
                stack.push((current, number));

                // 開始重新建立括號內的字串
                current = String::new();

                // 數字已經使用完畢，歸零準備處理下一段
                number = 0;
            } else if ch == ']' {
                // 遇到 ']'，代表目前這層括號處理完成。
                //
                // 取出進入這層括號前保存的字串和重複次數
                let (previous, repeat) = stack.pop().unwrap();

                // 將目前括號內的字串重複 repeat 次
                let repeated = current.repeat(repeat);

                // 把重複後的內容接回上一層字串
                current = previous + &repeated;
            } else {
                // 普通英文字母直接加入目前字串
                current.push(ch);
            }
        }

        // 所有括號處理完後，current 就是答案
        current
    }
}

#[cfg(test)]
#[path = "./decode_string_test.rs"]
mod tests;
