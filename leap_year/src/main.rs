use std::io;
fn check_leap(year_input: i32) -> bool {
    if year_input % 4 == 0 {
        if year_input % 100 == 0 {
            if year_input % 400 == 0 {
                true
            }
            else {
                false
            }
        }
        else {
            true
        }
    }
    else {
        false
    }
} 
fn main() {
    let mut count = 0;
    println!("Enter starting year: ");
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("error occured while reading");
    let year_int1: i32 = year.trim().parse::<i32>().unwrap();

    println!("Enter ending year: ");
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("error occured while reading");
    let year_int2: i32 = year.trim().parse::<i32>().unwrap();

    //let i = ;
    for i in year_int1..year_int2 {
        let x: bool = check_leap(i);
        if x {
            count = count + 1;
        }
        else {
        }
    }
    println!("No. of leap years: {}", count);
}