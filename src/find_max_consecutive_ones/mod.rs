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
