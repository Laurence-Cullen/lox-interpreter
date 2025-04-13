use crate::parser_utils::ws;
use crate::ws_separated;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{
    alpha1, alphanumeric0, alphanumeric1, line_ending, not_line_ending,
};
use nom::multi::many0;
use nom::number::complete::double;
use nom::sequence::delimited;
use nom::{IResult, Parser};

type Line = Vec<Token>;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Single character tokens
    I,
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
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Or,
    Class,
    Super,
    This,
    Fun,
    If,
    Else,
    True,
    False,
    Nil,
    Print,
    Return,
    Var,
    For,
    While,

    LineComment(String),
    Eof,
}

pub fn scan_lines(input: &str) -> Result<Vec<Line>, nom::Err<nom::error::Error<&str>>> {
    let mut lines: Vec<Line> = Vec::new();
    for line in input.lines() {
        let result = scan_line(line);

        // If result is not OK return error
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        let (remaining, tokens) = result?;

        if !remaining.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                remaining,
                nom::error::ErrorKind::NonEmpty,
            )));
        }
        lines.push(tokens);
    }
    Ok(lines)
}

/// Use nom to parse lines of lox code and return a vector of tokens.
pub fn scan_line(input: &str) -> IResult<&str, Vec<Token>> {
    many0(alt(ws_separated!((
        line_comment,
        keyword,
        identifier,
        number,
        string,
        two_char_token,
        single_char_token
    ))))
    .parse(input)
}

fn line_comment(input: &str) -> IResult<&str, Token> {
    let (remaining, comment) =
        delimited(tag("//"), not_line_ending, many0(line_ending)).parse(input)?;
    Ok((remaining, Token::LineComment(comment.to_string())))
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
        "(" => Token::I,
        ")" => Token::RightParen,
        "{" => Token::LeftBrace,
        "}" => Token::RightBrace,
        "," => Token::Comma,
        "." => Token::Dot,
        "-" => Token::Minus,
        "+" => Token::Plus,
        ";" => Token::Semicolon,
        "/" => Token::Slash,
        "*" => Token::Star,
        "=" => Token::Equal,
        _ => unreachable!(),
    };

    Ok((remaining, token_type))
}

fn two_char_token(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = alt((tag("!="), tag("=="), tag("<="), tag(">="))).parse(input)?;
    let token_type = match lexeme {
        "!=" => Token::BangEqual,
        "==" => Token::EqualEqual,
        "<=" => Token::LessEqual,
        ">=" => Token::GreaterEqual,
        _ => unreachable!(),
    };

    Ok((remaining, token_type))
}

fn identifier(input: &str) -> IResult<&str, Token> {
    let (remaining, (start, rest)) = (alpha1, alphanumeric0).parse(input)?;

    let lexeme = start.to_string() + rest;

    Ok((remaining, Token::Identifier(lexeme.to_string())))
}

fn string(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = delimited(tag("\""), is_not("\""), tag("\"")).parse(input)?;
    Ok((remaining, Token::String(lexeme.to_string())))
}

fn number(input: &str) -> IResult<&str, Token> {
    let (remaining, number) = double.parse(input)?;

    Ok((remaining, Token::Number(number)))
}

/// Return a token from the input string which is a Lox keyword
fn keyword(input: &str) -> IResult<&str, Token> {
    let (remaining, lexeme) = alphanumeric1(input)?;
    let token_type = match lexeme {
        "and" => Token::And,
        "class" => Token::Class,
        "else" => Token::Else,
        "false" => Token::False,
        "fun" => Token::Fun,
        "for" => Token::For,
        "if" => Token::If,
        "nil" => Token::Nil,
        "or" => Token::Or,
        "print" => Token::Print,
        "return" => Token::Return,
        "super" => Token::Super,
        "this" => Token::This,
        "true" => Token::True,
        "var" => Token::Var,
        "while" => Token::While,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alpha,
            )));
        }
    };
    Ok((remaining, token_type))
}

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
        assert_eq!(token, Token::And);
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
        assert_eq!(token, Token::Number(123.45));
        // assert_eq!(token.lexeme, "123.45");
    }

    #[test]
    fn test_identifier() {
        let input = "myVariable123";
        let (remaining, token) = identifier(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(token, Token::Identifier("myVariable123".to_string()));

        // Check invalid case
        let input = "123Invalid";
        let result = identifier(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_comment() {
        let input = "// This is a comment\n";
        let (remaining, comment) = line_comment(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            comment,
            Token::LineComment(" This is a comment".to_string())
        );
    }

    #[test]
    fn test_scan_line() {
        let input = "var x <= 10;";
        let tokens = scan_line(input);

        println!("{:?}", tokens);
    }

    #[test]
    fn test_scan_line_2() {
        let input = "var and2 = 10;";
        let (remaining, tokens) = scan_line(input).unwrap();

        let expected_tokens = vec![
            Token::Var,
            Token::Identifier("and2".to_string()),
            Token::Equal,
            Token::Number(10.0),
            Token::Semicolon,
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_scan_line_3() {
        let input = "andfunc for;  // This is a comment";
        let (remaining, tokens) = scan_line(input).unwrap();

        let expected_tokens = vec![
            Token::Identifier("andfunc".to_string()),
            Token::For,
            Token::Semicolon,
            Token::LineComment(" This is a comment".to_string()),
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
