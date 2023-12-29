use crate::lexer::ast::{Casing, Function};

pub fn transpile(functions: Vec<Function>) -> String {
    return functions.iter().fold(String::from(""), |mut acc, func| {
        match func {
            Function::Letter { casing, select } => {
                match casing {
                    Some(Casing::Upcase) => {
                        acc.push_str("[A-Z]");
                    }
                    Some(Casing::Downcase) | None => {
                        acc.push_str("[a-z]");
                    }
                }

                match select {
                    Some(num) => {
                        acc.push_str(format!("{{{num}}}").as_str());
                    }
                    None => {}
                }
            }
            Function::Letters { casing } => match casing {
                Some(Casing::Upcase) => {
                    acc.push_str("[A-Z]+");
                }
                Some(Casing::Downcase) | None => {
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
            Function::Number { select } => {
                acc.push_str("[0-9]");

                match select {
                    Some(num) => {
                        acc.push_str(format!("{{{num}}}").as_str());
                    }
                    None => {}
                }
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
