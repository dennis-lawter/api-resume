use thiserror::Error;

/// A specialized `Result` type for operations that can return errors.
///
/// This type is primarily used throughout your application to represent the
/// results of operations that might produce an error. It is a shorthand for
/// `std::result::Result` with a default error type of `color_eyre::Report`,
/// which provides enhanced error reporting capabilities.
pub type Result<T, E = color_eyre::Report> = std::result::Result<T, E>;

/// Represents the set of possible errors encountered in the application.
///
/// This enum provides a centralized place to capture all the potential errors
/// your application might encounter. By using the `thiserror` crate, it
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
}
