// pub fn quicksort(arr: &mut Vec<usize>, h: usize, l: usize) {
//     if l < h {
//         let piv = partition(&mut arr, l, h);
//         quicksort(&mut arr, h, piv + 1); // right
//         quicksort(&mut arr, piv - 1, l); // left
//     }
// }
// fn partition(arr: &mut Vec<usize>, l: usize, h: usize) -> u32 {
//     // [3,5,7,4]
//     let piv = h;
//     let i = l; // 0 -> 3
//     let high = l;
//     while i < h {
//         if arr[i] < arr[piv] {
//             // 0 ->3 h->4 -> 0->
//             swap(&mut arr[i], &mut arr[high]);
//             high += 1;
//         }
//         i += 1;
//     }
//     swap(&mut arr[piv], &mut arr[high]);
//     high
// }
// fn swap(one: &mut i32, two: &mut i32) {
//     let temp = one;
//     one = two;
//     two = temp;
// }
use std::fmt::Debug;
#[cfg(test)]
mod tests {
    #[test]
    fn test_quicksort() {
        use crate::quicksort::quick_sort;
        let arr = vec![3, 6, 5, 7];
        println!("{:?}", arr);
        let arr = quick_sort(&arr);
        println!("{:?}", arr);
    }
}
fn partition<T: PartialOrd + Clone + Debug>(
    collection: &mut [T],
    low: usize,
    high: usize,
) -> usize {
    let pivot = collection[high].clone();
    let (mut i, mut j) = (low as i64 - 1, high as i64 + 1);
    loop {
        'lower: loop {
            i += 1;
            if i > j || collection[i as usize] >= pivot {
                break 'lower;
            }
        }
        'upper: loop {
            j -= 1;
            if i > j || collection[j as usize] <= pivot {
                break 'upper;
            }
        }
        if i > j {
            return j as usize;
        }
        collection.swap(i as usize, j as usize);
    }
}
fn quick_sort_r<T: PartialOrd + Clone + Debug>(collection: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(collection, low, high);
        quick_sort_r(collection, low, pivot);
        quick_sort_r(collection, pivot + 1, high);
    }
}
pub fn quick_sort<T: PartialOrd + Clone + Debug>(collection: &[T]) -> Vec<T> {
    let mut result = collection.to_vec();
    quick_sort_r(&mut result, 0, collection.len() - 1);
    result
}
