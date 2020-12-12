use std::io::Write;

fn main() {
    print!("hello");
    println!("hello {}", "world");
    eprint!("hello {}", "error");
    eprintln!("hello");

    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, " is 123");
    dbg!(w);
}
