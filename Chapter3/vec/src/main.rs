fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    println!("{}", v2[0]);
    for element in &v1 {
        println!("{}", element);
    }
}
