//! Toptal Quiz
use log::trace;
/// # Toptal Interview Question
///
/// > Given an array : ["S>P","P>A","A>I","I>N"]
/// ## Example
///
/// ```
/// use adm::toptal_word::create_string;
/// fn main(){
///     println!("{:#?}",create_string(vec!["S>P", "P>A", "A>I", "I>N"]));
/// }
///```
use std::collections::HashMap;
/// # Create String
pub fn create_string(arr: Vec<String>) -> String {
    let mut map_for = HashMap::new();
    let mut map_rev = HashMap::new();
    if !arr.is_empty() {
        let arr: Vec<Vec<String>> = arr
            .iter()
            .map(|x| {
                let r: Vec<&str> = x.split(">").collect();
                map_for.insert(r[0].to_string(), r[1].to_string());
                map_rev.insert(r[1].to_string(), r[0].to_string());
                let sp_r = vec![r[0].to_string(), r[1].to_string()];
                sp_r
            })
            .collect();
        trace!("Forward Hashmap : {:?}", map_for);
        trace!("Backward Hashmap : {:?}", map_rev);
        format!(
            "{}{}",
            create_string_r(&arr[arr.len() / 2][0], &map_rev)
                .as_str()
                .chars()
                .rev()
                .collect::<String>(),
            create_string_r(&arr[(arr.len() / 2)][1], &map_for)
        )
    } else {
        String::from("")
    }
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
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_create_string_proptest(s in prop::collection::hash_set("[A-Z]",1..=20), v in prop::collection::hash_set("[A-Z]",1..=20))  {
            prop_assume!(s != v);
            use crate::toptal_word::create_string;
            let st = s.iter().zip(v.iter()).filter(|(x,y)|x!=y).map(|(x,y)|format!("{}>{}",x,y)).collect();
            println!("{:#?}",st);
            println!("{:#?}", create_string(st));
        }
    }
    #[test]
    fn test_create_string() {
        use crate::toptal_word::create_string;
        assert_eq!(
            "SPAIN",
            create_string(
                vec!["S>P", "P>A", "A>I", "I>N"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    fn test_create_string_lorem() {
        use crate::toptal_word::create_string;
        assert_eq!(
            "LOREM",
            create_string(
                vec!["R>E", "O>R", "E>M", "L>O"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            )
        );
    }
    //Edge Case when two identical Letters. Yet to solve this problem.
    // #[test]
    // fn test_create_string_success() {
    //     use crate::toptal_word::create_string;
    //     assert_eq!(
    //         "SUCCESS",
    //         create_string(
    //             vec!["S>U", "U>C", "C>C", "C>E", "E>S", "S>S"]
    //                 .iter()
    //                 .map(|x| x.to_string())
    //                 .collect::<Vec<String>>()
    //         )
    //     );
    // }
}
