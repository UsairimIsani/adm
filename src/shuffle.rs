pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let first_n = nums[0..(n as usize)].to_vec();
    let last_n = nums[(n as usize)..nums.len()].to_vec();
    first_n
        .into_iter()
        .zip(last_n)
        .map(|x| vec![x.0, x.1])
        .flatten()
        .collect()
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_shuffle_1() {
        use super::*;
        assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
    }
    #[test]
    fn test_shuffle_2() {
        use super::*;
        assert_eq!(
            shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
    }
    #[test]
    fn test_shuffle_3() {
        use super::*;
        assert_eq!(shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }
}
