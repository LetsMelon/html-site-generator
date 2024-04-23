use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IntoHtmlNodeError {
    #[error("{0:?}")]
    Io(#[from] std::io::Error),

    #[error("{0:?}")]
    InkjetError(#[from] inkjet::InkjetError),
}
