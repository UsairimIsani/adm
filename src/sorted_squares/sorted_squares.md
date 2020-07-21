# Leetcode Array Code.

## Usairim Implementaiton.

```rust
// This is My own implementation without any help from other Solutions
// Leetcode Says it takes 8ms and 2.1 MB of Resources, but
// After doing Criterion Bench marks it says this implementation is better
// than the best on leetcode.
pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    let mut a: Vec<i32> = a.into_iter().map(|x| x.pow(2)).collect();
    a.sort_unstable();
    a
}
```

## Leetcode Best Quick Sort Style Method

```rust
// It Takes 4ms on leetcode
// But is way slower on criterion? Why
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
```
