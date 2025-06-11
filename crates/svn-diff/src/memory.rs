//! routines for doing diffs on in-memory data

/// `source_tokens_t`
pub struct SourceToken {
    /// A token simply is an svn_string_t pointing to
    /// the data of the in-memory data source, containing
    /// the raw token text, with length stored in the strin
    tokens: Vec<String>,
    /// Next token to be consumed
    next_token: usize,
    /// The source, containing the in-memory data to be diffed
    source: String,
    /// The last token ends with a newline character (sequence)
    ends_without_eol: bool
}
