use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}