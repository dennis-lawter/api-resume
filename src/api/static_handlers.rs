use poem::handler;
use poem::http::StatusCode;
use poem::web::Html;

/// Handles requests to the root ("/") endpoint of the application.
///
/// This asynchronous function reads the content of the `static/index.html` file
/// and returns it as an `Html` response. If there's any issue reading the file,
/// an `INTERNAL_SERVER_ERROR` status code is returned with the associated error.
///
/// # Returns
/// - `Ok(Html(content))` if the file reading was successful.
/// - `Err(poem::Error)` if there was an issue reading the file.
#[handler]
pub async fn index() -> poem::Result<Html<String>, poem::Error> {
    let content = tokio::fs::read_to_string("static/index.html")
        .await
        .map_err(|err| poem::Error::new(err, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Html(content))
}
