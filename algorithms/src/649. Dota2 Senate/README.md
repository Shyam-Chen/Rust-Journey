# 649. Dota2 參議院 (Dota2 Senate)

在 Dota2 的世界裡，有兩個政黨：光輝黨 (Radiant) 和夜魘黨 (Dire)。

Dota2 參議院由兩黨的參議員組成。現在參議院想要就 Dota2 遊戲的改變做出決定。此更改的投票是基於回合的程序。在每一輪中，每位參議員可以行使以下兩項權利中的**一**項：

- **禁止一名參議員的權利**：一名參議員可以讓另一名參議員在本輪及後續所有輪次中失去所有權利。
- **宣布勝利**：如果參議員發現仍有投票權的參議員都來自同一黨派，他可以宣布勝利並決定遊戲的變化。

給定一個字串 `senate` 代表每位參議員所屬的政黨。字元 `'R'` 和 `'D'` 代表光輝黨 (Radiant) 和夜魘黨 (Dire)。那麼如果有 `n` 個參議員，則給定字串的大小將為 `n`。

輪次投票依照給定順序從第一位參議員開始到最後一位參議員。這一過程將持續到投票結束。所有失去權利的參議員都將在過程中被跳過。

假設每位參議員都足夠聰明，都會為自己的政黨採取最好的策略。預測哪一方最終會宣布勝利並改變 Dota2 的比賽格局。輸出應該是 `Radiant` 或 `Dire`。

範例 1：

```coffee
輸入: senate = "RD"
輸出: "Radiant"
說明:
第一位參議員來自 Radiant，他可以在第一輪禁止下一個參議員的權利。
第二位參議員不能再行使任何權利，因為他的權利被禁止。
在第二輪中，第一位參議員可以直接宣布勝利，因為他是參議院中唯一可以投票的人。
```

範例 2：

```coffee
輸入: senate = "RDD"
輸出: "Dire"
說明:
第一位參議員來自 Radiant，他可以在第一輪禁止下一個參議員的權利。
第二位參議員不能再行使任何權利，因為他的權利被禁止。
第三位參議員來自 Dire，他可以在第一輪禁止第一位參議員的權利。
在第二輪中，第三位參議員可以直接宣布勝利，因為他是參議院中唯一可以投票的人。
```

## 解題

```rs
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
```
