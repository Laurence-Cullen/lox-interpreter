use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, alphanumeric0};
use nom::number::complete::float;
use nom::sequence::delimited;
use nom::{IResult, Parser};
use nom::multi::many0;
use crate::parsers::ws;
use crate::ws_separated;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

/// Use nom to parse lines of lox code and return a vector of tokens.
pub fn scan_line(input: &str) -> Vec<Token> {
    // Check for comment line
    if let Ok((remaining, _)) = comment(input) {
        let mut tokens: Vec<Token> = Vec::new();
        return tokens
    };

    // Start while loop
    let mut remaining = input;

    let (remaining, mut tokens) = many0(alt(ws_separated!((
            keyword,
            identifier,
            number,
            string,
            two_char_token,
            single_char_token
        )))).parse(&mut remaining).unwrap();

    // Add EOF token
    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: String::new(),
        literal: String::new(),
        line: 0, // Placeholder for line number
    });
    tokens
}


fn comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("//"), is_not("\n"), tag("\n")).parse(input)
}

fn single_char_token(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = alt((
        tag("("),
        tag(")"),
        tag("{"),
        tag("}"),
        tag(","),
        tag("."),
        tag("-"),
        tag("+"),
        tag(";"),
        tag("/"),
        tag("*"),
        tag("="),
    ))
        .parse(input)?;

    let token_type = match lexeme {
        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "," => TokenType::Comma,
        "." => TokenType::Dot,
        "-" => TokenType::Minus,
        "+" => TokenType::Plus,
        ";" => TokenType::Semicolon,
        "/" => TokenType::Slash,
        "*" => TokenType::Star,
        "=" => TokenType::Equal,
        _ => unreachable!(),
    };

    Ok((
        remaining,
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: String::new(),
            line: 0, // Placeholder for line number
        },
    ))
}

fn two_char_token(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = alt((tag("!="), tag("=="), tag("<="), tag(">="))).parse(input)?;
    let token_type = match lexeme {
        "!=" => TokenType::BangEqual,
        "==" => TokenType::EqualEqual,
        "<=" => TokenType::LessEqual,
        ">=" => TokenType::GreaterEqual,
        _ => unreachable!(),
    };

    Ok((
        remaining,
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: String::new(),
            line: 0, // Placeholder for line number
        },
    ))
}

fn identifier(input: &str) -> IResult<&str, Token> {
    let (remaining, (start, rest)) = (alpha1, alphanumeric0).parse(input)?;

    let lexeme = start.to_string() + rest;

    Ok((
        remaining,
        Token {
            token_type: TokenType::Identifier,
            lexeme,
            literal: String::new(),
            line: 0, // Placeholder for line number
        },
    ))
}

fn string(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = delimited(tag("\""), is_not("\""), tag("\"")).parse(input)?;
    Ok((
        remaining,
        Token {
            token_type: TokenType::String,
            lexeme: lexeme.to_string(),
            literal: lexeme.to_string(),
            line: 0, // Placeholder for line number
        },
    ))
}

fn number(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = float.parse(input)?;

    Ok((
        remaining,
        Token {
            token_type: TokenType::Number,
            lexeme: lexeme.to_string(),
            literal: lexeme.to_string(),
            line: 0, // Placeholder for line number
        },
    ))
}

// fn literal(input: &str) -> IResult<&str, Token> {}

/// Return a token from the input string which is a Lox keyword
fn keyword(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = alpha1(input)?;
    let token_type = match lexeme {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "fun" => TokenType::Fun,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alpha,
            )));
        }
    };
    Ok((
        remaining,
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: String::new(),
            line: 0, // Placeholder for line number
        },
    ))
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword() {
        let input = "and";
        let result = keyword(input);
        assert!(result.is_ok());
        let (remaining, token) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(token.token_type, TokenType::And);
        assert_eq!(token.lexeme, "and");
    }

    #[test]
    fn test_invalid_keyword() {
        let input = "invalid";
        let result = keyword(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_number() {
        let input = "123.45";
        let (remaining, token) = number(input).unwrap();
        // assert!(result.is_ok());
        // let (remaining, token) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.lexeme, "123.45");
    }

    #[test]
    fn test_identifier() {
        let input = "myVariable123";
        let (remaining, token) = identifier(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.lexeme, "myVariable123");

        // Check invalid case
        let input = "123Invalid";
        let result = identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_comment() {
        let input = "// This is a comment\n";
        let (remaining, comment) = comment(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(comment, " This is a comment");
    }

    #[test]
    fn test_scan_line() {
        let input = "var x <= 10;";
        let tokens = scan_line(input);

        println!("{:?}", tokens);
    }

    #[test]
    fn test_scan_line_2() {
        let input = "var andx = 10;";
        let tokens = scan_line(input);

        let expected_tokens = vec![
            Token {
                token_type: TokenType::Var,
                lexeme: "var".to_string(),
                literal: "".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "andx".to_string(),
                literal: "".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_string(),
                literal: "".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "10".to_string(),
                literal: "10".to_string(),
                line: 0,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                literal: "".to_string(),
                line: 0,
            },
        ];
    }
}
