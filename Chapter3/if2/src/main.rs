fn main() {
    let number = -1;
    let result = if 0 <= number {
        number
    } else {
        -number
    };
    println!("{}", result)
}
