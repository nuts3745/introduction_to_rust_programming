fn main() {
    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }
}
