use color_eyre::Result;

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
    pub fn new() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            base_host: std::env::var("BASE_HOST")?,
            cargo_pkg_description: std::env::var("CARGO_PKG_DESCRIPTION")?,
            cargo_pkg_version: std::env::var("CARGO_PKG_VERSION")?,
            base_url: std::env::var("BASE_URL")?,
        })
    }
}
