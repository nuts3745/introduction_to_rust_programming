fn main() {
    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");
            break 'main;
            println!("sub loop end");
        }
        println!("main loop end");
    }
    loop {
        println!("main loop start");
        loop {
            println!("sub loop start");
            break;
            println!("sub loop end");
        }
        println!("main loop end");
        break;
    }
}