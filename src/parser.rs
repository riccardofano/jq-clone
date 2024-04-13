use winnow::combinator::{dispatch, fail, peek};
use winnow::token::{any, take};
use winnow::{PResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Identity,
}

fn parse_token(input: &mut &str) -> PResult<Token> {
    dispatch! {peek(any);
        '.' => take(1usize).value(Token::Identity),
        _ => fail
    }
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
}
