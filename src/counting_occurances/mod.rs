//! # Counting Occurances in Rust via Binary search
pub fn counting_occurances<T: PartialOrd + Clone + Debug>(col: &[T], key: T) -> Option<usize> {
    let mut result = col.to_vec();
    counting_occurances_r(&mut result, key, 0, col.len() - 1)
}
fn counting_occurances_r<T: PartialOrd + Clone + Debug>(
    col: &mut [T],
    key: T,
    low: usize,
    high: usize,
) -> Option<usize> {
    println!(
        "Low : {}, High :{}, Collection : {:?},key:{:?}",
        low, high, col, key
    );
    if low > high {
        println!("In low>high");
        Some(low)
    } else {
        println!("In Else");
        let middle = (low + high) / 2;
        let val = col[middle].clone();
        if val > key {
            println!("In Val > Key");
            println!(
                "Low : {}, High :{},val :{:?} ,Collection : {:?}",
                low, high, val, col
            );
            counting_occurances_r(col, key, low, middle)
        } else {
            println!("In Val < Key");
            counting_occurances_r(col, key, middle + 1, high)
        }
    }
}
use std::fmt::Debug;
#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search() {
        use crate::counting_occurances::counting_occurances;
        let arr = vec![3, 5, 5, 5, 6, 7];
        counting_occurances(&arr, 5);
        println!("{:?}", arr);
        // assert_eq!(Some(1, 3), binary_search(&arr, 5));
    }
}
