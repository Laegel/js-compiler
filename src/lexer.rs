#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Def,
    Extern,
    If,
    Then,
    Else,
    For,
    In,
    Binary,
    Unary,
    Const,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String)
}

use Token::*;

use regex::Regex;

pub fn tokenize(input: &str) -> Vec<Token> {
    // regex for commentaries (start with #, end with the line end)
    // let comment_re = Regex::new(r"(?m)#.*\n").unwrap();
    // let preprocessed = comment_re.replace_all(input, "\n");

    let mut result = Vec::new();

    // regex for token, just union of straightforward regexes for different token types
    // operators are parsed the same way as identifier and separated later
    let token_re = Regex::new(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)")).unwrap();

    for cap in token_re.captures_iter(input) {
        // let token = if cap.name("ident").is_some() {
        //     match cap.name("ident").unwrap() {
        //         "const" => Const,
        //         ident => Ident(ident.to_string())
        //     }
        // } else if cap.name("number").is_some() {
        //     match cap.name("number").unwrap().parse() {
        //         Ok(number) => Number(number),
        //         Err(_) => panic!("Lexer failed trying to parse number")
        //     }
        // } else if cap.name("delimiter").is_some() {
        //     Delimiter
        // } else if cap.name("oppar").is_some() {
        //     OpeningParenthesis
        // } else if cap.name("clpar").is_some() {
        //     ClosingParenthesis
        // } else if cap.name("comma").is_some() {
        //     Comma
        // } else {
        //     Operator(cap.name("operator").unwrap().to_string())
        // };

        // result.push(token)
    }

    result
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    use super::Token::*;
    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("const variable = 12"), vec![
            Const, Ident("variable".to_owned()), Operator("=".to_owned()), Number(12.0)
        ]);
    }
}
