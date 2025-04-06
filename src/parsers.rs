use nom::{character::complete::multispace0, sequence::delimited, Parser};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, Output, Function>(
    inner: Function,
) -> impl Parser<&'a str, Output = Output, Error = nom::error::Error<&'a str>>
where
    Function: Parser<&'a str, Output = Output, Error = nom::error::Error<&'a str>>,
{
    delimited(multispace0, inner, multispace0)
}

/// Takes in a tuple of parsers with different return types
/// and returns a tuple of parsers each wrapped with `ws`.
///
/// # Example
/// ```
/// use nom::character::complete::u32;
/// use nom::number::complete::float;
/// use nom::Parser;
/// use idf_parser::ws_separated;
/// use idf_parser::primitives::ws;
///
/// let input = "0 100.0 200.0 45.0";
///
/// let (remaining, (label, x, y, angle)) = ws_separated!((u32, float, float, float)).parse(input).unwrap();
/// ```
#[macro_export]
macro_rules! ws_separated {
    (($($parser:expr),+)) => {
        ($(ws($parser)),+)
    };
}
