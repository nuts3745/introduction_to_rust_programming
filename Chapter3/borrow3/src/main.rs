fn main() {
    let y;
    {
        let x = 5;
        y = &x;
        dbg!(x);
    }

    dbg!(y);
}
