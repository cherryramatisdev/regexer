use crate::lexer::tokens;

#[derive(Debug, Eq, PartialEq)]
pub enum Casing {
    Upcase,
    Downcase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Functions {
    Letter { casing: Casing },
    Letters { casing: Casing },
    Glob { rest: bool },
    Whitespace,
    Number,
    Numbers,
    Group(Box<Vec<Functions>>),
}

fn slice_until_end_func<'a>(
    tokens: &'a Vec<tokens::Token>,
    curr_idx: &'a usize,
) -> &'a [tokens::Token] {
    let tokens = &tokens[*curr_idx + 1..];
    if let Some(right_paren_pos) = tokens
        .iter()
        .position(|item| *item == tokens::Token::RightParen)
    {
        return &tokens[..right_paren_pos + 1];
    }

    return &[];
}

fn slice_until_end_group<'a>(
    tokens: &'a Vec<tokens::Token>,
    curr_idx: &'a usize,
) -> &'a [tokens::Token] {
    let tokens = &tokens[*curr_idx + 1..];

    for (idx, range) in tokens.windows(2).enumerate() {
        match range {
            [tokens::Token::RightParen, tokens::Token::RightParen]
            | [tokens::Token::Identifier(_), tokens::Token::RightParen] => {
                return &tokens[..idx + 1];
            }
            _ => continue,
        }
    }

    return &[];
}

pub fn tokenize(tokens: Vec<tokens::Token>) -> Vec<Functions> {
    let mut functions: Vec<Functions> = vec![];

    for (index, token) in tokens.iter().enumerate() {
        if let tokens::Token::Identifier(identifier) = token {
            match identifier.as_str() {
                "letter" => {
                    if slice_until_end_func(&tokens, &index)
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::True,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Functions::Letter {
                            casing: Casing::Upcase,
                        });
                    }

                    if slice_until_end_func(&tokens, &index)
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::False,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Functions::Letter {
                            casing: Casing::Downcase,
                        });
                    }
                }
                "letters" => {
                    if slice_until_end_func(&tokens, &index)
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::True,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Functions::Letters {
                            casing: Casing::Upcase,
                        });
                    }

                    if slice_until_end_func(&tokens, &index)
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("upcase".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::False,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Functions::Letters {
                            casing: Casing::Downcase,
                        });
                    }
                }
                "glob" => {
                    if slice_until_end_func(&tokens, &index)
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("rest".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::True,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Functions::Glob { rest: true });
                    }

                    if slice_until_end_func(&tokens, &index)
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("rest".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::False,
                            tokens::Token::RightParen,
                        ]
                    {
                        functions.push(Functions::Glob { rest: false });
                    }
                }
                "whitespace" => functions.push(Functions::Whitespace),
                "number" => functions.push(Functions::Number),
                "numbers" => functions.push(Functions::Numbers),
                "group" => {
                    if slice_until_end_group(&tokens, &index)
                        == [
                            tokens::Token::LeftParen,
                            tokens::Token::Parameter("rest".to_string()),
                            tokens::Token::Equal,
                            tokens::Token::True,
                            tokens::Token::RightParen,
                        ]
                    {
                        println!("aqui");
                    }

                    println!("AQUI -> {:?}", slice_until_end_group(&tokens, &index));
                }
                _ => {
                    println!("faltou quem {identifier}");
                }
            }
        }
    }

    return functions;
}
