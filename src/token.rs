use anyhow::bail;
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

fn apply_tokens(input: &Value, tokens: &[Token<'_>]) -> anyhow::Result<Value> {
    let mut output = input;

    for token in tokens {
        match token {
            Token::Identity => {}
            Token::Index(_) if !output.is_array() => {
                bail!("Can't index into non array value");
            }
            Token::Index(index) => {
                output = output.get(index).unwrap_or(&Value::Null);
            }
            Token::OptionalIndex(_) => todo!(),
            Token::Key(_) if !output.is_object() => {
                bail!("Can't access key of non object value");
            }
            Token::Key(key) => {
                output = output.get(key).unwrap_or(&Value::Null);
            }
            Token::OptionalKey(_) => todo!(),
            Token::Iterator => todo!(),
        }
    }

    Ok(output.to_owned())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn apply_identity_token() {
        let input = json!({"quotes": ["a", "b", "c"]});
        let tokens = vec![Token::Identity];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), input);
    }

    #[test]
    fn apply_index_to_array() {
        let input = json!([1, 2, 3]);
        let tokens = vec![Token::Index(2)];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), 3);
    }

    #[test]
    fn apply_out_of_bounds_index_to_array() {
        let input = json!([1, 2, 3]);
        let tokens = vec![Token::Index(3)];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), Value::Null);
    }

    #[test]
    fn apply_index_to_non_array() {
        let tokens = vec![Token::Index(2)];

        let input = json!("1");
        assert!(apply_tokens(&input, &tokens).is_err());
        let input = json!(1);
        assert!(apply_tokens(&input, &tokens).is_err());
        let input = json!({"hello": "world"});
        assert!(apply_tokens(&input, &tokens).is_err());
    }

    #[test]
    fn apply_index_to_array_chained() {
        let input = json!([[1, 2, 3], [4, 5, 6]]);
        let tokens = vec![Token::Index(1), Token::Index(0)];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), 4);
    }

    #[test]
    fn apply_index_to_array_chained_with_identity() {
        let input = json!([[1, 2, 3], [4, 5, 6]]);
        let tokens = vec![Token::Index(1), Token::Identity, Token::Index(0)];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), 4);
    }

    #[test]
    fn apply_key_to_object() {
        let input = json!({"hello": "world"});
        let tokens = vec![Token::Key("hello")];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), "world");
    }

    #[test]
    fn apply_non_existent_key_to_object() {
        let input = json!({"hello": "world"});
        let tokens = vec![Token::Key("missing")];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), Value::Null);
    }

    #[test]
    fn apply_key_to_non_object() {
        let tokens = vec![Token::Key("hello")];

        let input = json!("1");
        assert!(apply_tokens(&input, &tokens).is_err());
        let input = json!(1);
        assert!(apply_tokens(&input, &tokens).is_err());
        let input = json!([1, 2, 3, 4]);
        assert!(apply_tokens(&input, &tokens).is_err());
    }

    #[test]
    fn apply_key_to_object_chained() {
        let input = json!({"hello": {"world": 42}});
        let tokens = vec![Token::Key("hello"), Token::Key("world")];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), 42);
    }

    #[test]
    fn apply_chain_key_and_index_access() {
        let input = json!({"key": [1,2,3]});
        let tokens = vec![Token::Key("key"), Token::Index(0)];

        assert_eq!(apply_tokens(&input, &tokens).unwrap(), 1);
    }
}
