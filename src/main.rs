use autopp_inc::inc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    args[1..].iter().for_each(|arg| {
        let x: i64 = arg.parse().unwrap();
        println!("{}", inc(x));
    });
}
