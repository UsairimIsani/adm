# Leetcode Best

```rust
 pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut tempLength = 0;
        let mut maxLength = 0;
        let mut prevNum = nums[0];
        for i in 0..nums.len() {

            if nums[i] == 1 {
                tempLength += 1;
            }

            else if tempLength > maxLength {
                maxLength = tempLength;
                tempLength = 0;
            }

            else {
                tempLength = 0;
            }
        }
        if tempLength > maxLength {
            maxLength = tempLength;
        }
        maxLength
    }
```
