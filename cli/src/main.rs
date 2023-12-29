mod tui;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        let _ = tui::tui::call().unwrap();
        return
    }

    if args.len() > 1 {
        let first_argument = &args[1];
        let code = regexer::parse(first_argument.to_string());

        println!("{}", code);
    }
}
