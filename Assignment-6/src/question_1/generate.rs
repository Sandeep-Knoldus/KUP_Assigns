#[warn(dead_code)]
pub fn generate_substring(text: String) -> Vec<String> {
    let mut sub_string: Vec<String> = Vec::new();
    let mut substring: &str;
    let length = text.len();
    for loop1 in 0..length {
        for loop2 in loop1..length {
            substring = &text[loop1..(loop2 + 1)];
            sub_string.push(substring.to_string());
        }
    }
    sub_string
}
