//! Toptal Quiz
/// # Toptal Interview Question
///
/// > Given an array : ["S>P","P>A","A>I","I>N"]
use std::collections::HashMap;
pub fn create_string(arr: Vec<&str>) -> String {
    let mut map_for = HashMap::new();
    let mut map_rev = HashMap::new();
    let arr: Vec<Vec<String>> = arr
        .iter()
        .map(|x: &&str| {
            let r: Vec<&str> = x.split(">").collect();
            map_for.insert(r[0].to_string(), r[1].to_string());
            map_rev.insert(r[1].to_string(), r[0].to_string());
            let sp_r = vec![r[0].to_string(), r[1].to_string()];
            sp_r
        })
        .collect();
    format!(
        "{}{}",
        create_string_r(&arr[arr.len() / 2][0], &map_rev)
            .as_str()
            .chars()
            .rev()
            .collect::<String>(),
        create_string_r(&arr[(arr.len() / 2)][1], &map_for)
    )
}
fn create_string_r(key: &String, map: &HashMap<String, String>) -> String {
    let val = map.get(key);
    if val.is_none() {
        key.to_string()
    } else {
        format!("{}{}", key, create_string_r(val.unwrap(), map))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_string() {
        use crate::toptal_word::create_string;
        assert_eq!("SPAIN", create_string(vec!["S>P", "P>A", "A>I", "I>N"]));
    }
    #[test]
    fn test_create_string_lorem() {
        use crate::toptal_word::create_string;
        assert_eq!("LOREM", create_string(vec!["R>E", "O>R", "E>M", "L>O"]));
    }
    //Edge Case when two identical Letters
    // #[test]
    // fn test_create_string_success() {
    //     use crate::toptal_word::create_string;
    //     assert_eq!(
    //         "SUCCESS",
    //         create_string(vec!["S>U", "U>C", "C>C", "C>E", "E>S", "S>S"])
    //     );
    // }
}
