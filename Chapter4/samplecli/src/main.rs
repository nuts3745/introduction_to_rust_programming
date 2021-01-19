use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]

struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "NUMBER")]
    num: Option<i32>,

    #[clap(name = "FILE", default_value = "default.txt")]
    formula_file: String,
}

fn main() {
    let opts = Opts::parse();

    match opts.num {
        Some(num) => println!("Your number: {}", num),
        None => println!("No number specified."),
    }

     println!("File specified: {}", opts.formula_file);

    println!("Is verbosity specified?: {}", opts.verbose);
}
