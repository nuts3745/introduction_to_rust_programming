use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello, World!");
    });
}
