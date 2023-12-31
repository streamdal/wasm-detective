use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("error: {0}")]
    Error(String),

    #[error("match error: {0}")]
    MatchError(String),

    #[error("regex error: {source}")]
    RegexError {
        #[from]
        source: regex::Error,
    },

    #[error("missing match type: {0}")]
    MissingMatchType(i32),
}
