#[warn(unused_imports)]
#[cfg(test)]
use crate::question_1::generate::generate_substring;
use crate::question_1::search::search;
use crate::question_2::string_manipulate::output;

#[test]
fn generating_substring_test1() {
    assert_eq!(generate_substring("pa".to_string()), ["p", "pa", "a"]);
}

#[test]
fn search_substring_test1() {
    assert_eq!(search("Pankaj Chaudhury", "Cha"), "7".to_string());
}

#[test]
fn max_min() {
    assert_eq!(
        output(
            "jjdhid".to_string(),
            "ikjhjk".to_string(),
            "rtysgi".to_string()
        ),
        "itdsgk".to_string()
    );
}
