//! # ADM - Algorithms Design Manual
//!
//! To Understand Algorithms in depth
//!
//! To Understand the Rust Programming Language and Use it as a daily programming driver.
//!
//!
#![doc(html_playground_url = "https://play.rust-lang.org/")]
pub mod binary_search;
pub mod binary_tree;
pub mod capital_letters;
pub mod counting_occurances;
pub mod duplicate_zeroes;
pub mod find_max_consecutive_ones;
pub mod find_numbers;
pub mod quicksort;
pub mod single_num;
pub mod sorted_squares;
pub mod toptal_word;
pub fn init() {
    pretty_env_logger::try_init();
    // let _ = env_logger::builder()
    //     // Include all events in tests
    //     .filter_level(log::LevelFilter::max())
    //     // Ensure events are captured by `cargo test`
    //     .is_test(true)
    //     // Ignore errors initializing the logger if tests race to configure it
    //     .try_init();
}
