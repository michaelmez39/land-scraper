#[derive(Debug)]
pub enum ConversionError {
    MissingField(String),
    InvalidField(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(s) => write!(f, "Listing is missing field {}", s)?,
            Self::InvalidField(s) => write!(f, "Invalid field datatype for {}", s)?,
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum ProviderError {
    Conversion(serde_json::Error),
    Network(hyper::Error),
    Http(hyper::http::Error),
    Query,
}

impl From<hyper::Error> for ProviderError {
    fn from(e: hyper::Error) -> Self {
        ProviderError::Network(e)
    }
}

impl From<hyper::http::Error> for ProviderError {
    fn from(e: hyper::http::Error) -> Self {
        ProviderError::Http(e)
    }
}

impl From<serde_json::Error> for ProviderError {
    fn from(e: serde_json::Error) -> Self {
        ProviderError::Conversion(e)
    }
}
impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conversion(s) => write!(f, "Conversion failed: {}", s)?,
            Self::Http(s) => write!(f, "Http error, make sure backend url is correct: {}", s)?,
            Self::Network(s) => write!(f, "Error getting data: {}", s)?,
            Self::Query => write!(f, "Query was invalid")?,
        }
        Ok(())
    }
}

impl std::error::Error for ProviderError {}
impl std::error::Error for ConversionError {}
