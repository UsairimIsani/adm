/// Best Solution Running time 4ms
/// Uses a Quick Sort Approach
pub fn sorted_squares_quick(a: Vec<i32>) -> Vec<i32> {
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
/// My Implemented 97.69%tile - Running Time 8ms
pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut a: Vec<i32> = a.into_iter().map(|x| x.pow(2)).collect();
    a.sort_unstable();
    a
}
