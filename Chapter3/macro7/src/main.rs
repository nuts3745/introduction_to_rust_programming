fn main() {
    enum Emotion {
        Anger,
        Happy,
    }

    trait Emotional {
        fn get_happy(&mut self) -> String;
        fn get_anger(&mut self) -> String;
        fn tell_state(&self) -> String;
    }

    struct HappyPerson {
        name: String,
        state: Emotion,
    }

    impl Emotional for HappyPerson {
        fn get_anger(&mut self) -> String {
            unimplemented!()
        }
        fn get_happy(&mut self) -> String {
            format!("{} is always happy.", self.name)
        }
        fn tell_state(&self) -> String {
            todo!()
        }
    }

    fn f(x: usize) -> &'static str {
        match x {
            n if n * n % 3 == 0 => "3n",
            n if n * n % 3 == 1 => "3n+1 or 3n+2",
            _ => unreachable!(),
        }
    }

    let mut p = HappyPerson {
        name: "Takashi".to_string(),
        state: Emotion::Happy,
    };
    println!("{}", p.get_happy());
}
