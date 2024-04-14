#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Identity,
    Index(usize),
    Key(&'a str),
    OptionalIndex(usize),
    OptionalKey(&'a str),
    Iterator,
}
