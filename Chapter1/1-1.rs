fn main() {
    let x = 5;
    // println!は標準出力
    println!("number is: {}", x);
    x = 6; // 再代入しようとしているからエラー
    println!("number is: {}", x);
}