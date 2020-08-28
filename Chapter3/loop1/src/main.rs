fn main() {
    let mut count = 0;

    println!("\n loop{} \n", "");

    let _result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    println!("\n while{} \n", "");

    let mut count = 0;

    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    println!("\n for1{} \n", "");

    let _count: i32;

    for count in 0..10 {
        println!("count: {}", count);
    }

    println!("\n for2{} \n", "");

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    for element in &array {
        println!("element: {}", element);
    }
}
