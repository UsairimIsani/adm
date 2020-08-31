pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .scan(0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect()
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_running_sum_1() {
        use super::*;
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }
    #[test]
    fn test_running_sum_2() {
        use super::*;
        assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_running_sum_3() {
        use super::*;
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
}
