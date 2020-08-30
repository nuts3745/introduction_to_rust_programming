fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        // let z = &mut x;
        dbg!(y);
    }
    
    {
        let y = &x;
        let z = &x;
        let w = y + z;
        dbg!(y);
        dbg!(z);
        dbg!(w);
    }
}
