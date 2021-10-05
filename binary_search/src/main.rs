pub fn binary_search<T: PartialEq + PartialOrd>(item: &T, arr: &[T]) -> i32 {
    let mut idx_pos = -1;

    if arr.is_empty() {
        return idx_pos;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if &arr[mid] > item {
            right = mid - 1;
        } else if &arr[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if &arr[left] == item {
        idx_pos = left as i32;
        return idx_pos;
    } else {
        return idx_pos;
    }
}
fn main() {
    let index = binary_search(&43, &vec![25, 62, 29, 43, 77]);
    if index >= 0 {
        println!("Element found at position: {}", index);
    }
    else {
        println!("Element not found");
    }
    //println!("Position: {}", index);

    let index = binary_search(&855, &vec![25, 62, 29, 43, 77]);
    if index >= 0 {
        println!("Position: {}", index);
    }
    else {
        println!("Element not found");
    }
}