use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Identity,
    Index(usize),
    Key(&'a str),
    OptionalIndex(usize),
    OptionalKey(&'a str),
    Iterator,
}

fn apply_tokens(input: &Value, tokens: &[Token<'_>]) -> Value {
    let mut output = input;

    for token in tokens {
        match token {
            Token::Identity => {}
            Token::Index(_) => todo!(),
            Token::Key(_) => todo!(),
            Token::OptionalIndex(_) => todo!(),
            Token::OptionalKey(_) => todo!(),
            Token::Iterator => todo!(),
        }
    }

    output.to_owned()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn apply_identity_token() {
        let input = json!({"quotes": ["a", "b", "c"]});
        let tokens = vec![Token::Identity];

        assert_eq!(apply_tokens(&input, &tokens), input);
    }
}
