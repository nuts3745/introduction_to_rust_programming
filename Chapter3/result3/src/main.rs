fn main() {
    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }
    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func);
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func);
}
