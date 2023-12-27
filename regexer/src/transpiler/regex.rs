use crate::lexer::ast::{Casing, Function};

pub fn transpile(functions: Vec<Function>) -> String {
    return functions.iter().fold(String::from(""), |mut acc, func| {
        match func {
            Function::Letter { casing } => match casing {
                Casing::Upcase => {
                    acc.push_str("[A-Z]");
                }
                Casing::Downcase => {
                    acc.push_str("[a-z]");
                }
            },
            Function::Letters { casing } => match casing {
                Casing::Upcase => {
                    acc.push_str("[A-Z]+");
                }
                Casing::Downcase => {
                    acc.push_str("[a-z]+");
                }
            },
            Function::Glob { rest } => match rest {
                true => {
                    acc.push_str(".*");
                }
                false => {
                    acc.push_str(".");
                }
            },
            Function::Whitespace => {
                acc.push_str(r"\s");
            }
            Function::Number => {
                acc.push_str("[0-9]");
            }
            Function::Numbers => {
                acc.push_str("[0-9]+");
            }
            Function::Group(tokens) => {
                acc.push_str("(");
                acc.push_str(&transpile(tokens.to_vec()));
                acc.push_str(")");
            }
        };

        acc
    });
}
