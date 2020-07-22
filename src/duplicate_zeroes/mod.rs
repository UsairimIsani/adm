//! # Duplicate zeroes
//! [1,0,2,3,0,4,5,0] -> [1,0,0,2,3,0,0,4]
/// # Duplicate Zeros
/// ## Example 1:
/// ```
/// Input: [1,0,2,3,0,4,5,0]
/// Output: null
/// // Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
/// ```
/// ## Example 2:
/// ```
/// Input: [1,2,3]
/// Output: null
/// // Explanation: After calling your function, the input array is modified to: [1,2,3]
///```
/// ### Code
/// ```rust, run
/// use adm::duplicate_zeroes::duplicate_zeros;
/// let mut arr = vec![1,0,2,3,0,4,5,0];
/// duplicate_zeros(&mut arr);
/// assert_eq!(vec![1,0,0,2,3,0,0,4],arr);
/// ```

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == 0 {
            arr.insert(i + 1, 0);
            arr.pop();
            i += 2;
        } else {
            i += 1;
        }
    }
}
