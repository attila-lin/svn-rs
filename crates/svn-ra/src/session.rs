//! session

/* The RA session object. */
/// A repository access session.  This object is used to perform requests
/// to a repository, identified by a URL.
///
/// `svn_ra_session_t`
///
/// `libsvn_ra_svn/client.c`
pub struct SvnRaSession; //(Box<dyn RaSession>);

impl SvnRaSession {}

pub type SvnVersion = String;

/// `svn_ra__vtable_t`
pub trait RaSession {
    /// This field should always remain first in the vtable.
    fn get_version(&self) -> SvnVersion;

    /// Return a short description of the RA implementation, as a localized
    /// string.
    fn get_description(&self) -> &'static str;
    /// Return a list of actual URI schemes supported by this implementation.
    /// The returned array is NULL-terminated.
    fn get_schemes(&self) -> Vec<&'static str> {
        vec!["svn"]
    }
    /* See svn_ra_open5(). */
    // All fields in SESSION, except priv, have been initialized by the
    // time this is called.  SESSION->priv may be set by this function.
    fn open_session(corrected_url: &str, redirect_url: &str, session_url: &str) -> Self
    where
        Self: Sized;
}
/// Implementation of the `RaSession` trait for `SvnRaSession`.
impl RaSession for SvnRaSession {
    fn get_version(&self) -> SvnVersion {
        "1.0.0".to_string() // Placeholder version
    }

    fn get_description(&self) -> &'static str {
        if cfg!(feature = "_sasl") {
            r#"Module for accessing a repository using the svn network protocol.
              - with Cyrus SASL authentication"#
        } else {
            r#"Module for accessing a repository using the svn network protocol."#
        }
    }

    fn open_session(corrected_url: &str, redirect_url: &str, session_url: &str) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
