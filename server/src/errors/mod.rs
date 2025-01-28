use config::ConfigError;

#[derive(Debug)]
pub enum UnityCatalogError {
    Config(ConfigError),
}

pub type UnityCatalogResult<T> = Result<T, UnityCatalogError>;

impl std::fmt::Display for UnityCatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnityCatalogError::Config(error) => write!(f, "Configuration Error: {error}"),
        }
    }
}

impl std::error::Error for UnityCatalogError {}

impl From<ConfigError> for UnityCatalogError {
    fn from(value: ConfigError) -> Self {
        UnityCatalogError::Config(value)
    }
}
