use winnow::ascii::digit1;
use winnow::combinator::{delimited, dispatch, fail, peek};
use winnow::token::{any, take};
use winnow::{PResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Identity,
    Index(usize),
}

fn parse_token(input: &mut &str) -> PResult<Token> {
    dispatch! {peek(any);
        '.' => take(1usize).value(Token::Identity),
        '[' => parse_index.map(Token::Index),
        _ => fail
    }
    .parse_next(input)
}

fn parse_index(input: &mut &str) -> PResult<usize> {
    delimited('[', digit1, ']')
        .try_map(str::parse)
        .parse_next(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_identity_token() {
        let mut input = ".";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Identity)
    }

    #[test]
    fn parse_single_digit_index_token() {
        let mut input = "[1]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Index(1))
    }

    #[test]
    fn parse_multiple_digit_index_token() {
        let mut input = "[5280]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Index(5280))
    }
}
