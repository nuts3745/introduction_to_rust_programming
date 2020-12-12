fn main() {
    let s = concat!("A", "b2", 3);
    println!("{}", s);
    let s = String::from("Ab23");
    println!("{}", s);
    println!("{}", "");
    let s = format!("{}-{:?}", s, ("D", 5));
    println!("{}", s);
    let s = String::from("Ab23-(\"D\", 5)");
    println!("{}", s);
    println!("{}", "");
    let s = format!("{}{}", "abc", "def");
    println!("{}", s);
    let s = String::from("abcdef");
    println!("{}", s);
}
