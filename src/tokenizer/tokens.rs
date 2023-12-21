use std::{iter::Peekable, str::Chars};

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

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    LeftParen,
    RightParen,
    Whitespace,
    Equal,
    False,
    True,
    Pipe,
}

pub fn tokenize(line: String) -> Vec<Token> {
    let mut peeks = line.chars().peekable();

    let mut tokens: Vec<Token> = vec![];

    while let Some(&symbol) = peeks.peek() {
        match symbol {
            symbol if symbol.is_alphabetic() => {
                tokens.push(match read_string(&mut peeks).as_str() {
                    "True" => Token::True,
                    "False" => Token::False,
                    s => Token::Identifier(s.to_string()),
                });
            }
            '(' => {
                tokens.push(consume(Token::LeftParen, &mut peeks));
            }
            ')' => {
                tokens.push(consume(Token::RightParen, &mut peeks));
            }
            '|' => {
                tokens.push(consume(Token::Pipe, &mut peeks));
            }
            ' ' => {
                tokens.push(consume(Token::Whitespace, &mut peeks));
            }
            '=' => {
                tokens.push(consume(Token::Equal, &mut peeks));
            }
            _ => {
                panic!("Invalid token ->[{}]", symbol);
            }
        }
    }

    return tokens;
}

fn consume(token: Token, peeks: &mut Peekable<Chars<'_>>) -> Token {
    peeks.next();

    return token;
}

fn read_string(peeks: &mut Peekable<Chars<'_>>) -> String {
    let mut string = String::new();

    while let Some(&ch) = peeks.peek() {
        if ch.is_alphabetic() {
            string.push(ch);
            peeks.next();
        } else {
            break;
        }
    }

    return string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let sut =
            String::from("letter(upcase=True) | glob(rest=True) | whitespace | number");

        let expected = vec![
            Token::Identifier("letter".to_string()),
            Token::LeftParen,
            Token::Identifier("upcase".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("glob".to_string()),
            Token::LeftParen,
            Token::Identifier("rest".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("whitespace".to_string()),
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("number".to_string()),
        ];

        assert_eq!(tokenize(sut), expected);

        let sut =
            String::from("letters(upcase=True) | glob(rest=True) | whitespace | numbers");

        let expected = vec![
            Token::Identifier("letters".to_string()),
            Token::LeftParen,
            Token::Identifier("upcase".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("glob".to_string()),
            Token::LeftParen,
            Token::Identifier("rest".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("whitespace".to_string()),
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("numbers".to_string()),
        ];

        assert_eq!(tokenize(sut), expected);

        let sut =
            String::from("group(letter(upcase=True) | glob(rest=True) | whitespace) | number");

        let expected = vec![
            Token::Identifier("group".to_string()),
            Token::LeftParen,
            Token::Identifier("letter".to_string()),
            Token::LeftParen,
            Token::Identifier("upcase".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("glob".to_string()),
            Token::LeftParen,
            Token::Identifier("rest".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("whitespace".to_string()),
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("number".to_string()),
        ];

        assert_eq!(tokenize(sut), expected);

        let sut =
            String::from("group(letters(upcase=True) | glob(rest=True)) | whitespace | group(numbers)");

        let expected = vec![
            Token::Identifier("group".to_string()),
            Token::LeftParen,
            Token::Identifier("letters".to_string()),
            Token::LeftParen,
            Token::Identifier("upcase".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("glob".to_string()),
            Token::LeftParen,
            Token::Identifier("rest".to_string()),
            Token::Equal,
            Token::True,
            Token::RightParen,
            Token::RightParen,
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("whitespace".to_string()),
            Token::Whitespace,
            Token::Pipe,
            Token::Whitespace,
            Token::Identifier("group".to_string()),
            Token::LeftParen,
            Token::Identifier("numbers".to_string()),
            Token::RightParen,
        ];

        assert_eq!(tokenize(sut), expected);
    }
}
