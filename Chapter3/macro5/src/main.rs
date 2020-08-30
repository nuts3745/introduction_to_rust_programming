fn main() {
    println!("defined in file: {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));
}
