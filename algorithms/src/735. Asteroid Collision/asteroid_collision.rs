struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        // Stack 只保留「目前還存活」的小行星
        let mut stack: Vec<i32> = Vec::new();

        for asteroid in asteroids {
            // 表示目前這顆 asteroid 是否還活著
            let mut alive = true;

            // 只有以下情況可能發生碰撞：
            //
            // stack 頂端往右：top > 0
            // 目前 asteroid 往左：asteroid < 0
            //
            // 例如：
            //   5, -3 會碰撞
            //  -5,  3 不會碰撞，因為兩者方向相反
            while alive && asteroid < 0 {
                // 取得 stack 頂端的小行星
                let Some(&top) = stack.last() else {
                    // Stack 為空，沒有東西可以碰撞
                    break;
                };

                // 頂端往左，或目前 asteroid 往右，兩者不會碰撞
                if top < 0 {
                    break;
                }

                // 此時一定是：
                // top > 0，asteroid < 0
                let top_size = top as i64;
                let asteroid_size = (-asteroid) as i64;

                if top_size < asteroid_size {
                    // 目前 asteroid 比 top 大：
                    //
                    //   5, -8
                    //       ↑
                    //
                    // top 被撞毀，移除後繼續檢查更前面的 asteroid
                    stack.pop();
                } else if top_size == asteroid_size {
                    // 大小相同，兩者都會爆炸：
                    //
                    //   8, -8
                    stack.pop();
                    alive = false;
                } else {
                    // top 比目前 asteroid 大：
                    //
                    //   8, -5
                    //
                    // 目前 asteroid 被撞毀
                    alive = false;
                }
            }

            // 如果目前 asteroid 沒有被撞毀，放入 Stack
            if alive {
                stack.push(asteroid);
            }
        }

        stack
    }
}

#[cfg(test)]
#[path = "./asteroid_collision_test.rs"]
mod tests;
