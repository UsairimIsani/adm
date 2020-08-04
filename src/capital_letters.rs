pub fn detect_capital_use(word: String) -> bool {
    if word.is_empty() {
        false
    } else {
        let first = &word[0..1];
        let all_up_case = word[1..]
            .chars()
            .inspect(|x| {
                println!("all_up {}", x);
            })
            .all(is_uppercase_c);
        let all_low_case = word[1..]
            .chars()
            .inspect(|&x| {
                println!("all Low {}", x);
            })
            .all(is_lowercase_c);

        if is_uppercase(first) && all_up_case
            || is_uppercase(first) && all_low_case
            || is_lowercase(first) && all_low_case
        {
            true
        } else {
            false
        }
    }
}
fn is_uppercase(s: &str) -> bool {
    s.to_uppercase() == s
}
fn is_uppercase_c(c: char) -> bool {
    is_uppercase(c.to_string().as_str())
}
fn is_lowercase(s: &str) -> bool {
    s.to_lowercase() == s
}
fn is_lowercase_c(c: char) -> bool {
    is_lowercase(c.to_string().as_str())
}

#[cfg(test)]
mod tests {
    #[test]
    fn detect_capital_use_test1() {
        use super::*;
        assert_eq!(true, detect_capital_use(String::from("USA")));
    }
    #[test]
    fn detect_capital_use_test2() {
        use super::*;
        assert_eq!(false, detect_capital_use(String::from("FlaG")));
    }
    #[test]
    fn detect_capital_use_test3() {
        use super::*;
        assert_eq!(true, detect_capital_use(String::from("Google")));
    }
    #[test]
    fn detect_capital_use_test4() {
        use super::*;
        assert_eq!(true, detect_capital_use(String::from("ggg")));
    }
}
