pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> i32 {
    let mut idx_pos = -1; 
    for (idx, data) in arr.iter().enumerate() {
        if item == data {
            idx_pos = idx as i32;
            return idx_pos;
        }
    }
    idx_pos
}
fn main() {
    let index = linear_search(&25, &vec![77, 25, -17, 1, 48, 7]);
    if index >= 0 {
        println!("Element found at position: {}", index);
    }
    else {
        println!("Element not found");
    }

    let index = linear_search(&855, &vec![89, 90, 12, 9, 31, 6]);
    if index >= 0 {
        println!("Element found at position: {}", index);
    }
    else {
        println!("Element not found");
    }
}