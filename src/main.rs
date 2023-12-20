// TODO: Remove this shit
#![allow(dead_code)]

mod tokenizer;

fn main() {
    let a = "group(letter(upcase) | glob(rest=True) | whitespace) | number".to_string();
    println!("{:?}", tokenizer::tokens::parse(a));
}
