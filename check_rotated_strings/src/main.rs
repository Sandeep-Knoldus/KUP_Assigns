fn main() {
    println!("{}",is_rotated("abcd", "dcba"));
}
fn is_rotated(string1: &str, string2: &str) -> i32 {
    if string1.is_empty() || string2.is_empty() {
        return 0;
    }
    else if string1.len() != string2.len() {
        return 0;
    }
    else {
        let string_concat = format!("{}{}", string1, string2);
        if string_concat.contains(string2) {
            return 1;
        }
        else {
            return 0;
        }
    }
}
