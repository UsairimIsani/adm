//! Max Consecutive Ones - Leetcode
/// # Max Consecutive Ones - Leetcode
///
///
///
/// ## Leetcode  
pub fn find_max_consecutive_ones_leetcode(nums: Vec<i32>) -> i32 {
    let mut temp_length = 0;
    let mut max_length = 0;
    for i in 0..nums.len() {
        if nums[i] == 1 {
            temp_length += 1;
        } else if temp_length > max_length {
            max_length = temp_length;
            temp_length = 0;
        } else {
            temp_length = 0;
        }
    }
    if temp_length > max_length {
        max_length = temp_length;
    }
    max_length
}

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut current = vec![];
    for i in nums {
        if i == 1 {
            count += 1;
        } else {
            current.push(count);
            count = 0;
        }
    }
    current.push(count);
    current.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_max_cons_ones() {
        use super::find_max_consecutive_ones;
        assert_eq!(3, find_max_consecutive_ones(vec![1, 1, 0, 0, 1, 1, 1]));
    }
    #[test]
    fn test_find_max_cons_ones_leetcode() {
        use super::find_max_consecutive_ones_leetcode;
        assert_eq!(
            3,
            find_max_consecutive_ones_leetcode(vec![1, 1, 0, 0, 1, 1, 1])
        );
    }
}
