pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DbError(String),
    CustomError(String),
    IoError(std::io::Error),
}

impl From<&str> for Error {
    fn from(e: &str) -> Error {
        Error::CustomError(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IoError(e)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for Error {
    fn from(e: Box<dyn std::error::Error + Send + Sync + 'static>) -> Error {
        Error::CustomError(e.to_string())
    }
}
