use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello, World!");
    });

    dbg!(handle.join());
}
