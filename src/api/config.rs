use super::prelude::*;

/// Represents the configuration settings for the application.
pub struct Config {
    /// The database URL.
    pub database_url: String,
    /// The base host for the server.
    pub base_host: String,
    /// The description from the package's `Cargo.toml`.
    pub cargo_pkg_description: String,
    /// The version from the package's `Cargo.toml`.
    pub cargo_pkg_version: String,
    /// The base URL for the server.
    pub base_url: String,
}

impl Config {
    /// Creates a new `Config` instance by fetching and parsing various environment variables.
    ///
    /// # Returns
    /// Returns a `Result` wrapping the `Config` instance. Errors out if any of the required
    /// environment variables are not set or if there's an issue accessing them.
    pub fn new() -> ApplicationResult<Self> {
        Ok(Self {
            database_url: Self::load_env_or_fail("DATABASE_URL")?,
            base_host: Self::load_env_or_fail("BASE_HOST")?,
            cargo_pkg_description: Self::load_env_or_default(
                "CARGO_PKG_DESCRIPTION",
                "A RESTful API example providing my own professional resume.".to_owned(),
            ),
            cargo_pkg_version: Self::load_env_or_default("0.1.0", "".to_owned()),
            base_url: Self::load_env_or_fail("BASE_URL")?,
        })
    }

    fn load_env_or_fail(name: &str) -> ApplicationResult<String> {
        std::env::var(name).map_err(|_| Error::EnvVarMissing(name.to_string()))
    }

    fn load_env_or_default(name: &str, default: String) -> String {
        std::env::var(name).unwrap_or(default)
    }
}
