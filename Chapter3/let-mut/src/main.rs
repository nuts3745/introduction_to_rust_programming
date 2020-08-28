fn main() {
    let immut_val = 10;
    let mut mut_val = 20;
    println!("{}", immut_val);
    println!("{}", mut_val);

    mut_val += immut_val;

    let v1: u64 = 10;
    let v2 = 10u64;

    println!("{}", mut_val);
    println!("{}", v1);
    println!("{}", v2);
}
