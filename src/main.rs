// TODO: Remove this shit
#![allow(dead_code)]

mod lexer;
mod transpiler;

fn main() {
    let input =
        String::from("group(letters(upcase=True) | glob(rest=True)) | whitespace | group(numbers)");

    println!(
        "{}",
        transpiler::regex::transpile(lexer::ast::parse(lexer::tokens::tokenize(input)))
    );
}
