use autopp_inc::inc;
use clap::Parser;

#[derive(Parser)]
struct Args {
    numbers: Vec<i64>,
}

fn main() {
    let args = Args::parse();
    args.numbers.into_iter().for_each(|x| {
        println!("{}", inc(x));
    });
}
