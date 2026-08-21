# 2095. 刪除鏈結串列的中間節點 (Delete the Middle Node of a Linked List)

給定一個鏈結串列 `head`。**刪除**鏈結串列的**中間節點**，並回傳修改後的鏈結串列 `head`。

長度為 `n` 鏈結串列的中間節點是從頭數起第 `⌊n / 2⌋` 個節點 (索引從 0 開始)，其中 `⌊x⌋` 表示小於或等於 `x` 的最大整數。

- 對於 `n` = `1`、`2`、`3`、`4` 和 `5`，中間節點分別為 `0`、`1`、`1`、`2` 和 `2`。

範例 1：

```ts
                  ❌
(1) → (3) → (4) → (7) → (1) → (2) → (6)
 0     1     2     3     4     5     6
```

```coffee
輸入: head = [1,3,4,7,1,2,6]
輸出: [1,3,4,1,2,6]
說明:
上圖表示給定的鏈結串列。節點的索引寫在下面。
由於 n = 7，值為 7 的節點 3 是中間節點，以紅色叉標示。
刪除該節點後，我們返回新鏈結串列。
```

範例 2：

```ts
            ❌
(1) → (2) → (3) → (4)
 0     1     2     3
```

```coffee
輸入: head = [1,2,3,4]
輸出: [1,2,4]
說明:
上圖表示給定的鏈結串列。
對於 n = 4，值為 3 的節點 2 是中間節點，以紅色叉標示。
```

範例 3：

```ts
      ❌
(2) → (1)
 0     1
```

```coffee
輸入: head = [2,1]
輸出: [2]
說明:
上圖表示給定的鏈結串列。
對於 n = 2，值為 1 的節點 1 是中間節點，以紅色叉標示。
值為 2 的節點 0 是刪除節點 1 後剩下的唯一節點。
```

## 解題

```rs
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 先取得 head 的所有權，後面需要修改 linked list
        let mut head = head;

        // 計算 linked list 的長度
        let mut len = 0;
        let mut current = head.as_ref();

        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }

        // 題目通常保證至少有一個節點。
        // 若只有一個節點，刪除後直接回傳空 linked list。
        if len == 1 {
            return None;
        }

        // 定義的 middle index 是 len / 2。
        //
        // 例如：
        // len = 5，middle index = 2，刪除第 3 個節點
        // len = 4，middle index = 2，刪除第 3 個節點
        //
        // 因此需要找到 middle node 的前一個節點。
        let middle_index = len / 2;

        // 因為 len >= 2，所以 head 一定存在
        let mut current = head.as_mut().unwrap();

        // 移動到 middle node 的前一個節點
        for _ in 0..middle_index - 1 {
            current = current.next.as_mut().unwrap();
        }

        // current.next 就是要刪除的 middle node。
        //
        // take() 會把 current.next 取出，並將 current.next 設為 None。
        // 接著將被刪除節點的 next 接回來即可。
        //
        // 原本的：current -> middle -> next
        // 修改後：current -----------> next
        let middle = current.next.take().unwrap();
        current.next = middle.next;

        head
    }
}
```
