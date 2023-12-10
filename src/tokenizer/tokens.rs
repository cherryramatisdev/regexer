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

pub fn parse(line: String) -> Vec<Functions> {
    return line
        .split(" | ")
        .filter(|input| !input.is_empty())
        .map(|input| {
            let mut parts = input.split('(');
            let function_name = parts.next().unwrap_or_default().trim();

            let second_part = parts.next().unwrap_or_default();
            let second_part = second_part.trim_end_matches(')');

            let parameters: Vec<&str> = second_part.split(',').map(str::trim).collect();

            return (function_name, parameters);
        })
        .map(
            |(function_name, parameters)| match (function_name, parameters.as_slice()) {
                ("letter", ["upcase"]) => Functions::Letter {
                    casing: Casing::Upcase,
                },
                ("letters", ["upcase"]) => Functions::Letters {
                    casing: Casing::Upcase,
                },
                ("glob", ["rest=True"]) => Functions::Glob { rest: true },
                ("glob", ["rest=False"]) => Functions::Glob { rest: false },
                ("whitespace", _) => Functions::Whitespace,
                ("number", _) => Functions::Number,
                ("numbers", _) => Functions::Numbers,
                _ => todo!(),
            },
        )
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokens;

    #[test]
    fn parse() {
        let input = String::from("letter(upcase) | glob(rest=True) | whitespace | number");

        assert_eq!(
            tokens::parse(input),
            vec![
                tokens::Functions::Letter {
                    casing: tokens::Casing::Upcase
                },
                tokens::Functions::Glob { rest: true },
                tokens::Functions::Whitespace,
                tokens::Functions::Number,
            ]
        );

        let input = String::from("letters(upcase) | glob(rest=True) | whitespace | numbers");

        assert_eq!(
            tokens::parse(input),
            vec![
                tokens::Functions::Letters {
                    casing: tokens::Casing::Upcase
                },
                tokens::Functions::Glob { rest: true },
                tokens::Functions::Whitespace,
                tokens::Functions::Numbers,
            ]
        );
    }
}
