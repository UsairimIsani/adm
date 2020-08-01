//! # Binary Search
/// # Binary Search
///
/// This implementation required good grip on the use of slices.
///
/// Next thing to learn is slices in rust.
///
///
///
/// # BInary Search
use log::debug;
pub fn binary_search<T: PartialOrd + Clone + Debug>(col: &[T], key: T) -> Option<T> {
    let mut result = col.to_vec();
    binary_search_r(&mut result, key, 0, col.len() - 1)
}
fn binary_search_r<T: PartialOrd + Clone + Debug>(
    col: &mut [T],
    key: T,
    low: usize,
    high: usize,
) -> Option<T> {
    debug!(
        "Low : {}, High :{}, Collection : {:?},key:{:?}",
        low, high, col, key
    );
    if low > high {
        debug!("In low>high");
        None
    } else {
        debug!("In Else");
        let middle = (low + high) / 2;
        let val = col[middle].clone();
        if val == key {
            debug!("In Key Found");
            Some(val)
        } else if val > key {
            debug!("In Val > Key");
            debug!(
                "Low : {}, High :{},val :{:?} ,Collection : {:?}",
                low, high, val, col
            );
            binary_search_r(col, key, low, middle)
        } else {
            debug!("In Val < Key");
            binary_search_r(col, key, middle + 1, high)
        }
    }
}
use std::fmt::Debug;
#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search() {
        // crate::init();
        use crate::binary_search::binary_search;
        let arr = vec![3, 5, 6, 7];
        println!("{:?}", arr);
        assert_eq!(Some(5), binary_search(&arr, 5));
        assert_eq!(Some(3), binary_search(&arr, 3));
        assert_eq!(None, binary_search(&arr, 9));
    }
}
