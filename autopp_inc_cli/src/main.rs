use clap::Parser;

#[derive(Parser)]
struct Args {
    input: i64,
}

fn main() {
    let args = Args::parse();
    println!("{}", autopp_inc::inc(args.input));
}
