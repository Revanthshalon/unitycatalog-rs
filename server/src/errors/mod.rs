use config::ConfigError;

#[derive(Debug)]
pub enum UnityCatalogError {
    Config(ConfigError),
    Unsupported(String),
}

pub type UnityCatalogResult<T> = Result<T, UnityCatalogError>;

impl std::fmt::Display for UnityCatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnityCatalogError::Config(error) => write!(f, "Configuration Error: {error}"),
            UnityCatalogError::Unsupported(message) => write!(f, "Unsupported Error: {message}"),
        }
    }
}

impl std::error::Error for UnityCatalogError {}

impl From<ConfigError> for UnityCatalogError {
    fn from(value: ConfigError) -> Self {
        UnityCatalogError::Config(value)
    }
}
