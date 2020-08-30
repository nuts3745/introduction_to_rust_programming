fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        let z = &mut x;
        dbg!(y);
        dbg!(z);
    }
    
    {
        let y = &x;
        let z = &mut x;
        dbg!(y);
        dbg!(z);
    }
}
