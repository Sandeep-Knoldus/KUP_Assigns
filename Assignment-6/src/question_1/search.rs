#[warn(dead_code)]
pub fn search(text: &str, path: &str) -> String {
    let path_length = path.len();
    let text_length = text.len();

    let char1: Vec<char> = path.chars().collect();
    let char2: Vec<char> = text.chars().collect();

    for loop1 in 0..=text_length - path_length + 1 {
        let mut loop2 = 0;
        while loop2 < path_length {
            if char2[loop1 + loop2] != char1[loop2] {
                break;
            }
            loop2 += 1;
        }
        if loop2 == path_length {
            return loop1.to_string();
        }
    }
    return "".to_string();
}
