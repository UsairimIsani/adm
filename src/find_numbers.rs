pub fn find_numbers_leetcode(nums: Vec<i32>) -> i32 {
    return nums
        .iter()
        .map(|&num| {
            let mut counter = 0;
            let mut current = num;
            while current != 0 {
                counter += 1;
                current /= 10;
            }
            return counter;
        })
        .fold(0, |cum, v| cum + (v % 2 == 0) as i32);
}
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .filter(|x| x.to_string().len() % 2 == 0)
        .collect::<Vec<i32>>()
        .len() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_numbers() {
        use super::find_numbers;
        assert_eq!(2, find_numbers(vec![1212, 121, 12221, 12]));
    }
    #[test]
    fn test_find_numbers_leetcode() {
        use super::find_numbers_leetcode;
        assert_eq!(2, find_numbers_leetcode(vec![1212, 121, 12221, 12]));
    }
}
