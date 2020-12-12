struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    fn say_age(&self) {
        println!("I am {} yeat(s) old.", self.age);
    }
}


fn main() {
    let p = Person {
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name();
    p.say_age();
}
