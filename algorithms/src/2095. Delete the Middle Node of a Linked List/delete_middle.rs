struct Solution;

use crate::utils::list_node::ListNode;

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

#[cfg(test)]
#[path = "./delete_middle_test.rs"]
mod tests;
