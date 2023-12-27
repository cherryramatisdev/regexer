// TODO: Remove this shit
#![allow(dead_code)]

mod lexer;

fn main() {
    let input =
        String::from("group(letters(upcase=True) | glob(rest=True)) | whitespace | group(numbers)");

    println!(
        "MAIN -> {:?}",
        lexer::ast::parse(lexer::tokens::tokenize(input))
    );
}
