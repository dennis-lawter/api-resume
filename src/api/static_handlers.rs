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

/// Handles requests to the ("/neofetch") endpoint of the application.
///
/// Loads a pre-rendered neofetch output (showing INTERNAL_SERVER_ERROR on failure).
/// The neofetch output is converted from ANSI to HTML.
///
/// # Returns
/// - `Ok(Html(content))` if the file reading and conversion was successful.
/// - `Err(poem::Error)` if there was an issue reading or converting the file.
#[handler]
pub async fn neofetch() -> poem::Result<Html<String>, poem::Error> {
    let ansi_content = tokio::fs::read_to_string("static/neofetch.txt")
        .await
        .map_err(|err| poem::Error::new(err, StatusCode::INTERNAL_SERVER_ERROR))?;
    let ansi_content_no_break = ansi_content
        .replace("[20A", "INSERT_A_LINE_BREAK_HERE")
        .replace("[?25l[?7l", "");
    let ansi_converted = ansi_to_html::convert_escaped(&ansi_content_no_break)
        .map_err(|err| poem::Error::new(err, StatusCode::INTERNAL_SERVER_ERROR))?;
    let html_content = ansi_converted.replace("\n", "<br>").replace(" ", "&nbsp;");
    let html_page = format!(
        "<html><head><style>body{{background-color:#000;color:#fff;}} .mono{{display:inline-block;font-family:monospace;margin:1em;vertical-align:top;}}</style></head><body><div class='mono'>{}</div></body></html>",
        html_content.replace("INSERT_A_LINE_BREAK_HERE", "</div><div class='mono'>")
    );
    Ok(Html(html_page))
}
