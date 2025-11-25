use analog_ai_meaning_engine::build_meaning_curve;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: meaning-engine <file>");
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input");
    let curve = build_meaning_curve(&input);

    println!("{}", curve);
}
