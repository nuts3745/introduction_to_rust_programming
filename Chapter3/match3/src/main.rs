fn main() {
    let result: Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        },
    };
}
