//! # Sorted Squares
/// # Sorted Square Leetcode
///
///
///
///
///
pub fn sorted_squares_leetcode(a: Vec<i32>) -> Vec<i32> {
    let mut forward = a.iter().peekable();
    let mut back = a.iter().rev().peekable();
    let mut res: Vec<i32> = vec![0; a.len()];

    for n in res.iter_mut().rev() {
        let fp = **forward.peek().unwrap();
        let bp = **back.peek().unwrap();
        if fp.abs() > bp.abs() {
            *n = fp.pow(2);
            forward.next();
        } else {
            *n = bp.pow(2);
            back.next();
        }
    }
    res
}
/// # Sorted Squares
///
///
///
///
pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut a: Vec<i32> = a.into_iter().map(|x| x.pow(2)).collect();
    a.sort_unstable();
    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sorted_squares() {
        // use super::sorted_squares;
    }
    #[test]
    fn test_sorted_squares_leetcode() {
        // use super::sorted_squares;
    }
}
