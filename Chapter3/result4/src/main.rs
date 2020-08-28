fn main() {
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("code: {}", code);
        Ok(100)
    }
}
