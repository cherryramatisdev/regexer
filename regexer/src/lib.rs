mod lexer;
mod transpiler;

pub fn parse(input: String) -> String {
    let tokens = lexer::tokens::tokenize(input);
    let ast = lexer::ast::parse(tokens);

    return transpiler::regex::transpile(ast);
}
