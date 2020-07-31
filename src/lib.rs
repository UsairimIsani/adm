//! # ADM - Algorithms Design Manual
//! To Understand Algorithms in depth
#![doc(html_playground_url = "https://play.rust-lang.org/")]
pub mod binary_search;
pub mod binary_tree;
pub mod counting_occurances;
pub mod duplicate_zeroes;
pub mod quicksort;
pub mod single_num;
pub mod sorted_squares;
pub mod toptal_word;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
