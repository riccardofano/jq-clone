use anyhow::{ensure, Context};
use serde_json::Value;
use winnow::combinator::iterator;

use crate::{parser::parse_token, token::apply_tokens};

mod parser;
mod token;

pub fn apply_filter(input: &str, filter: Option<&str>) -> anyhow::Result<String> {
    let json: Value = serde_json::from_str(input).context("Failed to parse JSON")?;

    let filter = filter.unwrap_or(".");
    let mut it = iterator(filter, parse_token);

    let tokens = it.collect::<Vec<_>>();
    let (remaining, _) = it
        .finish()
        .map_err(|e| anyhow::anyhow!("Failed to parse filter: {e:?}"))?;

    ensure!(remaining.is_empty(), "Failed to parse the whole filter");

    let filtered = apply_tokens(&json, &tokens)?;

    token::token_output_to_string(filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_output_to_string() {
        let input = r#"{"hello": [1,2,3]}"#;
        let filter = ".hello[]";

        assert_eq!(
            apply_filter(input, Some(filter)).unwrap(),
            "1\n2\n3".to_owned()
        )
    }
}
