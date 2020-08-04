pub fn is_power_of_four(num: i32) -> bool {
    if num != 0 {
        (num as f64).log(4.0).floor() == (num as f64).log(4.0).ceil()
    } else {
        false
    }
}
pub fn is_power_of_four_leetcode(num: i32) -> bool {
    is_power_of_two(num) && (num & 0b1010101010101010101010101010101 != 0)
}

fn is_power_of_two(n: i32) -> bool {
    n > 0 && n & (-n) == n
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_power_of_four_1() {
        use super::*;
        assert_eq!(false, is_power_of_four(0));
    }
    #[test]
    fn is_power_of_four_2() {
        use super::*;
        assert_eq!(true, is_power_of_four(1));
    }
    #[test]
    fn is_power_of_four_3() {
        use super::*;
        assert_eq!(true, is_power_of_four(16));
    }
    #[test]
    fn is_power_of_four_4() {
        use super::*;
        assert_eq!(false, is_power_of_four());
    }
}
