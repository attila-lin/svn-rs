#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid MIME type: {0}")]
    InvalidMimeType(String),
}

/// `svn_mime_type_validate`
pub fn mime_type_validate(mime_type: &str) -> Result<(), Error> {
    mime_type
        .parse::<mime::Mime>()
        .map_err(|_| Error::InvalidMimeType(mime_type.into()))?;
    Ok(())
}

/// Return FALSE iff @a mime_type is a textual type.
///
/// All mime types that start with "text/" are textual, plus some special
/// cases (for example, "image/x-xbitmap").
///
/// `svn_mime_type_is_binary`
pub fn is_binary(mime_type: &str) -> Result<bool, Error> {
    let mime: mime::Mime = mime_type
        .parse()
        .map_err(|_| Error::InvalidMimeType(mime_type.into()))?;
    if mime.type_() == mime::TEXT {
        return Ok(false);
    }
    Ok(true)
}
