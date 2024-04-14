use anyhow::bail;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Identity,
    Index(usize),
    Key(&'a str),
    OptionalIndex(usize),
    OptionalKey(&'a str),
    Iterator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    Single(Value),
    Multiple(Vec<Output>),
}

pub fn apply_tokens(input: &Value, tokens: &[Token<'_>]) -> anyhow::Result<Output> {
    let mut output = input;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Identity => {}
            Token::Index(_) if !output.is_array() => {
                bail!("Can't index into non array value");
            }
            Token::Index(index) | Token::OptionalIndex(index) => {
                output = output.get(index).unwrap_or(&Value::Null);
            }
            Token::Key(_) if !output.is_object() => {
                bail!("Can't access key of non object value");
            }
            Token::Key(key) | Token::OptionalKey(key) => {
                output = output.get(key).unwrap_or(&Value::Null);
            }
            Token::Iterator => match output {
                Value::Array(array) => {
                    let transformed = array
                        .iter()
                        .map(|v| apply_tokens(v, &tokens[i + 1..]))
                        .collect::<Result<Vec<Output>, _>>()?;

                    return Ok(Output::Multiple(transformed));
                }
                Value::Object(map) => {
                    let transformed = map
                        .into_iter()
                        .map(|(_, v)| apply_tokens(v, &tokens[i + 1..]))
                        .collect::<Result<Vec<Output>, _>>()?;

                    return Ok(Output::Multiple(transformed));
                }
                // NOTE(riki): In jq this would be an error but I like it like this
                _ => return Ok(Output::Single(output.to_owned())),
            },
        }
    }

    Ok(Output::Single(output.to_owned()))
}

pub fn token_output_to_string(output: Output) -> anyhow::Result<String> {
    let string = match output {
        Output::Single(value) => serde_json::to_string_pretty(&value)?,
        Output::Multiple(values) => values
            .into_iter()
            .map(token_output_to_string)
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"),
    };

    Ok(string)
}

pub fn print_output(output: Output) {
    match output {
        Output::Single(v) => println!("{v}"),
        Output::Multiple(values) => values.into_iter().for_each(print_output),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn apply_identity_token() {
        let input = json!({"quotes": ["a", "b", "c"]});
        let tokens = vec![Token::Identity];

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(input)
        );
    }

    #[test]
    fn apply_index_to_array() {
        let input = json!([1, 2, 3]);
        let tokens = vec![Token::Index(2)];

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(json!(3))
        );
    }

    #[test]
    fn apply_out_of_bounds_index_to_array() {
        let input = json!([1, 2, 3]);
        let tokens = vec![Token::Index(3)];

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
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

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(json!(4))
        );
    }

    #[test]
    fn apply_index_to_array_chained_with_identity() {
        let input = json!([[1, 2, 3], [4, 5, 6]]);
        let tokens = vec![Token::Index(1), Token::Identity, Token::Index(0)];

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(json!(4))
        );
    }

    #[test]
    fn apply_key_to_object() {
        let input = json!({"hello": "world"});
        let tokens = vec![Token::Key("hello")];

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(json!("world"))
        );
    }

    #[test]
    fn apply_non_existent_key_to_object() {
        let input = json!({"hello": "world"});
        let tokens = vec![Token::Key("missing")];

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
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

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(json!(42))
        );
    }

    #[test]
    fn apply_chain_key_and_index_access() {
        let input = json!({"key": [1,2,3]});
        let tokens = vec![Token::Key("key"), Token::Index(0)];

        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(json!(1))
        );
    }

    #[test]
    fn apply_optional_index_to_non_array() {
        let tokens = vec![Token::OptionalIndex(1)];

        let input = json!("1");
        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
        let input = json!(1);
        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
        let input = json!({"hello": "world"});
        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
    }

    #[test]
    fn apply_optional_key_to_non_object() {
        let tokens = vec![Token::OptionalKey("hello")];

        let input = json!("1");
        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
        let input = json!(1);
        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
        let input = json!([1, 2, 3, 4]);
        assert_eq!(
            apply_tokens(&input, &tokens).unwrap(),
            Output::Single(Value::Null)
        );
    }

    #[test]
    fn apply_iterator() {
        let tokens = vec![Token::Iterator];

        let input = json!([[1, [2, 3]], [4, [5, 6]], [7, [8, 9]]]);
        let res = apply_tokens(&input, &tokens).unwrap();

        let expected = Output::Multiple(vec![
            Output::Single(json!([1, [2, 3]])),
            Output::Single(json!([4, [5, 6]])),
            Output::Single(json!([7, [8, 9]])),
        ]);

        assert_eq!(res, expected);
    }

    #[test]
    fn apply_iterator_chained() {
        let tokens = vec![Token::Iterator, Token::Iterator];

        let input = json!([[1, [2, 3]], [4, [5, 6]], [7, [8, 9]]]);
        let res = apply_tokens(&input, &tokens).unwrap();

        let expected = Output::Multiple(vec![
            Output::Multiple(vec![
                Output::Single(json!(1)),
                Output::Single(json!([2, 3])),
            ]),
            Output::Multiple(vec![
                Output::Single(json!(4)),
                Output::Single(json!([5, 6])),
            ]),
            Output::Multiple(vec![
                Output::Single(json!(7)),
                Output::Single(json!([8, 9])),
            ]),
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn apply_iterator_chained_twice() {
        let tokens = vec![Token::Iterator, Token::Iterator, Token::Iterator];

        let input = json!([[1, [2, 3]], [4, [5, 6]], [7, [8, 9]]]);
        let res = apply_tokens(&input, &tokens).unwrap();

        let expected = Output::Multiple(vec![
            Output::Multiple(vec![
                Output::Single(json!(1)),
                Output::Multiple(vec![Output::Single(json!(2)), Output::Single(json!(3))]),
            ]),
            Output::Multiple(vec![
                Output::Single(json!(4)),
                Output::Multiple(vec![Output::Single(json!(5)), Output::Single(json!(6))]),
            ]),
            Output::Multiple(vec![
                Output::Single(json!(7)),
                Output::Multiple(vec![Output::Single(json!(8)), Output::Single(json!(9))]),
            ]),
        ]);
        assert_eq!(res, expected);
    }

    #[test]
    fn apply_iterator_on_object() {
        let tokens = vec![Token::Iterator];

        let input = json!({"hello": "a", "world": "b"});
        let res = apply_tokens(&input, &tokens).unwrap();

        let expected =
            Output::Multiple(vec![Output::Single(json!("a")), Output::Single(json!("b"))]);

        assert_eq!(res, expected);
    }

    #[test]
    fn apply_iterator_on_array_in_object() {
        let tokens = vec![Token::Iterator, Token::Iterator];

        let input = json!({"hello": ["a", "b"], "world": ["c"]});
        let res = apply_tokens(&input, &tokens).unwrap();

        let expected = Output::Multiple(vec![
            Output::Multiple(vec![Output::Single(json!("a")), Output::Single(json!("b"))]),
            Output::Multiple(vec![Output::Single(json!("c"))]),
        ]);

        assert_eq!(res, expected);
    }
}
