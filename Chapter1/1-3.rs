fn main() {
    let objective: Option<i32> = Some(1);
    match objective {
        Some(x) if x % 2 == 0 => println!("偶数です: {}", x),
        Some(x) => println!("奇数です: {}", x),
        None => println!("値がありません"),
    }
}