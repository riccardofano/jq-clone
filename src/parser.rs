use winnow::ascii::digit1;
use winnow::combinator::{alt, delimited, dispatch, fail, terminated};
use winnow::token::{any, take_till};
use winnow::{PResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token<'a> {
    Identity,
    Index(usize),
    Key(&'a str),
    OptionalIndex(usize),
    OptionalKey(&'a str),
    Iterator,
}

fn parse_token<'a>(input: &mut &'a str) -> PResult<Token<'a>> {
    dispatch! {any;
        '.' => alt((
            terminated(parse_key, '?').map(Token::OptionalKey),
            parse_key.map(Token::Key),
            "".value(Token::Identity)
        )),
        '[' => alt((
            terminated(parse_index, '?').map(Token::OptionalIndex),
            parse_index.map(Token::Index),
            terminated(parse_key_string, '?').map(Token::OptionalKey) ,
            parse_key_string.map(Token::Key),
            "]".value(Token::Iterator)
        )),
        _ => fail
    }
    .parse_next(input)
}

fn parse_index(input: &mut &str) -> PResult<usize> {
    terminated(digit1, ']')
        .try_map(str::parse)
        .parse_next(input)
}

fn parse_key_string<'a>(input: &mut &'a str) -> PResult<&'a str> {
    terminated(delimited('"', parse_key, '"'), ']').parse_next(input)
}

fn parse_key<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_till(1.., |c: char| c == '.' || c == '[' || c == '"' || c == '?')
        .recognize()
        .parse_next(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_identity_token() {
        let mut input = ".";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Identity);
        assert!(input.is_empty());
    }

    #[test]
    fn parse_single_digit_index_token() {
        let mut input = "[1]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Index(1));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_multiple_digit_index_token() {
        let mut input = "[5280]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Index(5280));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_single_digit_optional_index_token() {
        let mut input = "[1]?";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::OptionalIndex(1));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_multiple_digit_optional_index_token() {
        let mut input = "[5280]?";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::OptionalIndex(5280));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_key_array_index() {
        let mut input = "[\"key\"]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Key("key"));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_key_array_index_with_digits() {
        let mut input = "[\"key123\"]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Key("key123"));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_key_array_index_with_digits_prefixed() {
        let mut input = "[\"123key\"]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Key("123key"));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_optional_key_array_index() {
        let mut input = "[\"key\"]?";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::OptionalKey("key"));
        assert!(input.is_empty());
    }

    #[test]
    fn parse_optional_key_array_index_with_digits() {
        let mut input = "[\"key123\"]?";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::OptionalKey("key123"));
        assert!(input.is_empty());
    }

    #[test]
    fn array_key_index_without_quotes() {
        let mut input = "[key]";
        let output = parse_token.parse_next(&mut input);
        assert!(output.is_err());
    }

    #[test]
    fn key_is_one_letter() {
        let mut input = "a";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "a");
        assert!(input.is_empty());
    }

    #[test]
    fn key_is_two_letter() {
        let mut input = "ab";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "ab");
        assert!(input.is_empty());
    }

    #[test]
    fn key_is_one_letter_one_digit() {
        let mut input = "a1";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "a1");
        assert!(input.is_empty());
    }

    #[test]
    fn key_is_one_letter_one_underscore() {
        let mut input = "a_";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "a_");
        assert!(input.is_empty());
    }

    #[test]
    fn key_is_one_letter_multiple_digits() {
        let mut input = "a1034803141";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "a1034803141");
        assert!(input.is_empty());
    }

    #[test]
    fn key_is_one_letter_multiple_underscores() {
        let mut input = "a________";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "a________");
        assert!(input.is_empty());
    }

    #[test]
    fn key_is_underscore_separated() {
        let mut input = "a_b_c_1_2";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "a_b_c_1_2");
        assert!(input.is_empty());
    }

    #[test]
    fn key_is_unicode_characters() {
        let mut input = "🎉🎆✨";
        let output = parse_key.parse_next(&mut input).unwrap();
        assert_eq!(output, "🎉🎆✨");
        assert!(input.is_empty());
    }

    #[test]
    fn parse_key_dot_notation() {
        let mut input = ".quote";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Key("quote"));
        assert_eq!(input, "");
    }

    #[test]
    fn parse_key_dot_notation_stops_at_open_bracket() {
        let mut input = ".quote[]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Key("quote"));
        assert_eq!(input, "[]");
    }

    #[test]
    fn parse_key_dot_notation_stops_at_dot() {
        let mut input = ".quote.quote";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Key("quote"));
        assert_eq!(input, ".quote");
    }

    #[test]
    fn parse_optional_key_dot_notation() {
        let mut input = ".quote?";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::OptionalKey("quote"));
        assert_eq!(input, "");
    }

    #[test]
    fn parse_optional_key_dot_notation_stops_at_open_bracket() {
        // TODO: This might not be valid syntax
        let mut input = ".quote?[]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::OptionalKey("quote"));
        assert_eq!(input, "[]");
    }

    #[test]
    fn parse_optional_key_dot_notation_stops_at_dot() {
        let mut input = ".quote?.quote";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::OptionalKey("quote"));
        assert_eq!(input, ".quote");
    }

    #[test]
    fn parse_iterator_token() {
        let mut input = ".quote[]";
        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Key("quote"));

        let output = parse_token.parse_next(&mut input).unwrap();
        assert_eq!(output, Token::Iterator);
        assert!(input.is_empty());
    }
}
