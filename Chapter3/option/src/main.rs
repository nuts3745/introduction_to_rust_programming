fn main() {
    pub enum Option<T> {
        None,
        Some(T),
    }

    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    };
}