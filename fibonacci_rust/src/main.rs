fn main() {
    let mut loop1 = 1;
    let mut first = 0;
    let mut second = 1;
    let mut result;

    while loop1 < 10 {
        println!("{}", first);
        result = first + second;
        first = second;
        second = result;
        loop1 = loop1 + 1;
    }
}