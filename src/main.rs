mod fizzbuzz;

use clap::Parser;
use fizzbuzz::FizzBuzz;

/// Hello FizzBuzz
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number to calculate FizzBuzz for
    #[arg(short, long, default_value_t = 30)]
    number: i32,
}

fn main() {
    let args = Args::parse();

    for number in 1..=args.number {
        let fizzbuzz = FizzBuzz::new(number);
        println!("{}", fizzbuzz.fizzbuzz())
    }
}
