fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("Error: No argument provided.");
        std::process::exit(1);
    }

    if args.len() > 1 {
        let first_argument = &args[1];
        let code = regexer::parse(first_argument.to_string());

        println!("{}", code);
    }
}
