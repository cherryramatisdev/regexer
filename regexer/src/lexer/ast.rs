use crate::lexer::tokens;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Casing {
    Upcase,
    Downcase,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Function {
    Letter {
        casing: Option<Casing>,
        select: Option<u32>,
    },
    Letters {
        casing: Option<Casing>,
    },
    Glob {
        rest: bool,
    },
    Whitespace,
    Number {
        select: Option<u32>,
    },
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

fn find_int_parameter(tokens: &[tokens::Token], parameter: String) -> Option<u32> {
    let founded = tokens.windows(3).find(|window| {
        matches!(
            window,
            [
                tokens::Token::Parameter(param),
                tokens::Token::Equal,
                tokens::Token::Int(_),
            ] if *param == parameter
        )
    });

    if let Some(inner_tokens) = founded {
        if let [tokens::Token::Parameter(_), tokens::Token::Equal, tokens::Token::Int(i)] =
            inner_tokens[..]
        {
            return Some(i);
        }
    }

    return None;
}

fn find_casing_parameter(tokens: &[tokens::Token], parameter: String) -> Option<Casing> {
    let upcase = tokens.windows(3).any(|window| {
        matches!(
            window,
            [
                tokens::Token::Parameter(param),
                tokens::Token::Equal,
                tokens::Token::True,
            ] if *param == parameter
        )
    });

    let downcase = tokens.windows(3).any(|window| {
        matches!(
            window,
            [
                tokens::Token::Parameter(param),
                tokens::Token::Equal,
                tokens::Token::False,
            ] if *param == parameter
        )
    });

    if upcase && !downcase {
        return Some(Casing::Upcase);
    }

    if downcase && !upcase {
        return Some(Casing::Downcase);
    }

    return None;
}

// TODO: define a `consume` function to not keep repeating the peeks_tokens.next() all the time
pub fn parse(tokens: Vec<tokens::Token>) -> Vec<Function> {
    let mut peeks_tokens = tokens.clone().into_iter().enumerate().peekable();
    let mut functions: Vec<Function> = vec![];

    while let Some((index, token)) = peeks_tokens.peek() {
        if let tokens::Token::Identifier(identifier) = token {
            match identifier.as_str() {
                "letter" | "letters" => {
                    let (func_tokens, right_pos_idx) = slice_until_end_func(&tokens, &index);

                    let casing = find_casing_parameter(func_tokens, "upcase".to_string());
                    let select = find_int_parameter(func_tokens, "select".to_string());

                    if identifier == "letter" {
                        functions.push(Function::Letter {
                            casing: casing.clone(),
                            select: select.clone(),
                        });
                    }

                    if identifier == "letters" {
                        functions.push(Function::Letters {
                            casing: casing.clone(),
                        });
                    }

                    peeks_tokens.nth(right_pos_idx + 1);
                }
                "number" => {
                    let (func_tokens, right_pos_idx) = slice_until_end_func(&tokens, &index);

                    let select = find_int_parameter(func_tokens, "select".to_string());

                    functions.push(Function::Number {
                        select: select.clone(),
                    });

                    peeks_tokens.nth(right_pos_idx + 1);
                }
                "numbers" => {
                    peeks_tokens.next();
                    functions.push(Function::Numbers)
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

                    peeks_tokens.nth(right_pos_idx + 1);
                }
                "whitespace" => {
                    peeks_tokens.next();
                    functions.push(Function::Whitespace)
                }
                "group" => {
                    let (group_tokens, right_pos_idx) = slice_until_end_group(&tokens, &index);

                    let tokens = parse(group_tokens.to_vec());

                    functions.push(Function::Group(Box::new(tokens)));

                    peeks_tokens.nth(right_pos_idx + 1);
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
                        casing: Some(Casing::Upcase),
                    },
                    Function::Glob { rest: true }
                ])),
                Function::Whitespace,
                Function::Group(Box::new(vec![Function::Numbers]))
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
                    casing: Some(Casing::Upcase),
                    select: None,
                },
                Function::Letter {
                    casing: Some(Casing::Downcase),
                    select: None
                },
                Function::Glob { rest: true },
                Function::Glob { rest: false },
                Function::Whitespace,
                Function::Number { select: None },
            ]
        );

        let input = String::from("letters(upcase=True) | glob(rest=True) | whitespace | numbers");

        assert_eq!(
            parse(tokens::tokenize(input)),
            vec![
                Function::Letters {
                    casing: Some(Casing::Upcase),
                },
                Function::Glob { rest: true },
                Function::Whitespace,
                Function::Numbers,
            ]
        );
    }

    #[test]
    fn test_select_parameters() {
        let input = String::from(
            "letter(select=3, upcase=True) | letters(upcase=False) | glob(rest=True) | glob(rest=False) | whitespace | number(select=99)",
        );

        assert_eq!(
            parse(tokens::tokenize(input)),
            vec![
                Function::Letter {
                    casing: Some(Casing::Upcase),
                    select: Some(3),
                },
                Function::Letters {
                    casing: Some(Casing::Downcase),
                },
                Function::Glob { rest: true },
                Function::Glob { rest: false },
                Function::Whitespace,
                Function::Number { select: Some(99) },
            ]
        );
    }
}
