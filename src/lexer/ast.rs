use crate::lexer::tokens;

#[derive(Debug, Eq, PartialEq)]
pub enum Casing {
    Upcase,
    Downcase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Function {
    Letter { casing: Casing },
    Letters { casing: Casing },
    Glob { rest: bool },
    Whitespace,
    Number,
    Numbers,
    Group(Box<Vec<Function>>),
}

fn slice_until_end_func<'a>(
    tokens: &'a Vec<tokens::Token>,
    curr_idx: &'a usize,
) -> (&'a [tokens::Token], usize) {
    let tokens = &tokens[*curr_idx + 1..];
    if let Some(right_paren_pos) = tokens
        .iter()
        .position(|item| *item == tokens::Token::RightParen)
    {
        return (&tokens[..right_paren_pos + 1], right_paren_pos);
    }

    return (&[], 0);
}

fn slice_until_end_group<'a>(
    tokens: &'a Vec<tokens::Token>,
    curr_idx: &'a usize,
) -> (&'a [tokens::Token], usize) {
    let tokens = &tokens[*curr_idx + 1..];

    for (idx, range) in tokens.windows(2).enumerate() {
        match range {
            [tokens::Token::RightParen, tokens::Token::RightParen]
            | [tokens::Token::Identifier(_), tokens::Token::RightParen] => {
                return (&tokens[..idx + 1], idx);
            }
            _ => continue,
        }
    }

    return (&[], 0);
}

// TODO: define a `consume` function to not keep repeating the peeks_tokens.next() all the time
pub fn parse(tokens: Vec<tokens::Token>) -> Vec<Function> {
    let mut peeks_tokens = tokens.clone().into_iter().enumerate().peekable();
    let mut functions: Vec<Function> = vec![];

    while let Some((index, token)) = peeks_tokens.peek() {
        if let tokens::Token::Identifier(identifier) = token {
            match identifier.as_str() {
                "letter" => {
                    let (func_tokens, right_pos_idx) = slice_until_end_func(&tokens, &index);
                    if func_tokens
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::True,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Function::Letter {
                            casing: Casing::Upcase,
                        });
                    }

                    if func_tokens
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::False,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Function::Letter {
                            casing: Casing::Downcase,
                        });
                    }

                    peeks_tokens.nth(right_pos_idx);
                }
                "letters" => {
                    let (func_tokens, right_pos_idx) = slice_until_end_func(&tokens, &index);
                    if func_tokens
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::True,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Function::Letters {
                            casing: Casing::Upcase,
                        });
                    }

                    if func_tokens
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::False,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Function::Letters {
                            casing: Casing::Downcase,
                        });
                    }

                    peeks_tokens.nth(right_pos_idx);
                }
                "glob" => {
                    let (func_tokens, right_pos_idx) = slice_until_end_func(&tokens, &index);
                    if func_tokens
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("rest".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::True,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Function::Glob { rest: true });
                    }

                    if func_tokens
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("rest".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::False,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Function::Glob { rest: false });
                    }

                    peeks_tokens.nth(right_pos_idx);
                }
                "whitespace" => {
                    peeks_tokens.next();
                    functions.push(Function::Whitespace)
                }
                "number" => {
                    peeks_tokens.next();
                    functions.push(Function::Number)
                }
                "numbers" => {
                    peeks_tokens.next();
                    functions.push(Function::Numbers)
                }
                "group" => {
                    let (group_tokens, right_pos_idx) = slice_until_end_group(&tokens, &index);

                    let tokens = parse(group_tokens.to_vec());

                    functions.push(Function::Group(Box::new(tokens)));

                    peeks_tokens.nth(right_pos_idx);
                }
                _ => {
                    panic!("Invalid identifier -> [{identifier}]");
                }
            }
        } else {
            peeks_tokens.next();
        }
    }

    return functions;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokens;

    #[test]
    fn test_grouped_tokens() {
        let input = String::from(
            "group(letters(upcase=True) | glob(rest=True)) | whitespace | group(numbers)",
        );

        assert_eq!(
            parse(tokens::tokenize(input)),
            vec![
                Function::Group(Box::new(vec![
                    Function::Letters {
                        casing: Casing::Upcase
                    },
                    Function::Glob { rest: true }
                ])),
                Function::Whitespace,
                Function::Group(Box::new(vec![Function::Numbers])),
                Function::Numbers
            ]
        );
    }

    #[test]
    fn test_basic_tokenize() {
        let input = String::from(
            "letter(upcase=True) | letter(upcase=False) | glob(rest=True) | glob(rest=False) | whitespace | number",
        );

        assert_eq!(
            parse(tokens::tokenize(input)),
            vec![
                Function::Letter {
                    casing: Casing::Upcase
                },
                Function::Letter {
                    casing: Casing::Downcase
                },
                Function::Glob { rest: true },
                Function::Glob { rest: false },
                Function::Whitespace,
                Function::Number,
            ]
        );

        let input = String::from("letters(upcase=True) | glob(rest=True) | whitespace | numbers");

        assert_eq!(
            parse(tokens::tokenize(input)),
            vec![
                Function::Letters {
                    casing: Casing::Upcase
                },
                Function::Glob { rest: true },
                Function::Whitespace,
                Function::Numbers,
            ]
        );
    }
}
