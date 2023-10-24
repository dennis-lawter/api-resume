use poem::http::StatusCode;
use thiserror::Error;

/// A specialized `Result` type for operations that can return errors.
///
/// This type is primarily used throughout the application to represent the
/// results of operations that might produce an error. It is a shorthand for
/// `std::result::Result` with an error type bound by the system's managed errors.
pub type ApplicationResult<T> = std::result::Result<T, Error>;

/// A specialized `Result` type for operations that can return errors.
///
/// This type is primarily used throughout the application to represent the
/// results of operations that might produce any error. It is a shorthand for
/// `std::result::Result` with a default error type of `color_eyre::Report`,
/// which provides enhanced error reporting capabilities.
pub type EyreResult<T, E = color_eyre::Report> = std::result::Result<T, E>;

/// Represents the set of possible errors encountered in the application.
///
/// This enum provides a centralized place to capture all the potential errors
/// the application might encounter. By using the `thiserror` crate, it
/// also provides user-friendly error messages and integrates well with the
/// `color_eyre` error reporting.
#[derive(Error, Debug)]
pub enum Error {
    /// An error that originates from the `sqlx` crate when working with databases.
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    /// An error that occurs during database migrations using the `sqlx` crate.
    #[error(transparent)]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    /// An error that occurs when expecting a query result.
    #[error("Not found")]
    NotFound,

    /// An error with the env file.
    #[error("Environment variable '{0}' is missing")]
    EnvVarMissing(String),
}
/// Converts application errors to poem errors, including HTTP response codes
impl From<Error> for poem::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::SqlxError(_) => poem::Error::new(error, StatusCode::INTERNAL_SERVER_ERROR),
            Error::MigrationError(_) => poem::Error::new(error, StatusCode::INTERNAL_SERVER_ERROR),
            Error::NotFound => poem::Error::new(error, StatusCode::NOT_FOUND),
            Error::EnvVarMissing(_) => poem::Error::new(error, StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
