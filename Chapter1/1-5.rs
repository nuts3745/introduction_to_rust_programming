fn main() {
    let objective: Option<i32> = Some(1);
    match objective {
        Some(x) if x % 2 == 0 => println!("偶数です"),
        Some(x) => println!("奇数です"),
        // Noneの処理が足りてないのでエラー
    }
}