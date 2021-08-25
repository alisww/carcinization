macro_rules! err_from {
    ($type:path, $variant:path) => {
        impl From<$type> for CarcinizationError {
            fn from(err: $type) -> CarcinizationError {
                $variant(err)
            }
        }
    };
}

#[derive(Debug)]
pub enum CarcinizationError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::error::Error),
}

impl std::error::Error for CarcinizationError {}

err_from!(reqwest::Error, CarcinizationError::ReqwestError);
err_from!(serde_json::error::Error, CarcinizationError::SerdeJsonError);

impl std::fmt::Display for CarcinizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CarcinizationError::ReqwestError(e) => write!(f, "reqwest error: {}", e),
            CarcinizationError::SerdeJsonError(e) => write!(f, "serde_json error: {}", e),
        }
    }
}

pub type CarcinizationResult<T> = Result<T, CarcinizationError>;
