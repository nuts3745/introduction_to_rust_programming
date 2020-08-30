fn main() {
    let mut x = 5;
    let y = &x;
    let z = &mut x;
    dbg!(z);
    dbg!(x);
}
