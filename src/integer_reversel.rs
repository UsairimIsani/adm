pub fn reverse(mut x: i32) -> i32 {
    let mut result = 0;

    while x != 0 {
        let digit = x % 10;
        x = x / 10;

        if (result > i32::MAX / 10 || result == i32::MAX / 10 && digit >= i32::MAX % 10)
            || (result < i32::MIN / 10 || result == i32::MIN / 10 && digit <= i32::MIN % 10)
        {
            return 0;
        }

        result = result * 10 + digit
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_integer() {
        let x = 1534236469;
        assert_eq!(reverse(x), 0)
    }
}
