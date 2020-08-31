fn main() {
    struct Color {
        r: i32,
        g: i32,
        b: i32,
    }

    let a = Color{r:255, g:255, b:255};
    let b = a;
    println!("{} {} {}", b.r, b.g, b.b);
}
