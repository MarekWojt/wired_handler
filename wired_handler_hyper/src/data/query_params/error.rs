use thiserror::Error;

use crate::data::http_error::HttpError;

#[derive(Debug, Error)]
pub enum GetQueryParamsError {
    #[error("{0}")]
    Parse(#[from] serde_html_form::de::Error),
}

impl From<GetQueryParamsError> for HttpError {
    fn from(value: GetQueryParamsError) -> Self {
        match value {
            GetQueryParamsError::Parse(err) => Self::bad_request(err.to_string()),
        }
    }
}
