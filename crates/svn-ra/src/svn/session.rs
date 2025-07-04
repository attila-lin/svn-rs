//! session

use std::collections::HashMap;

use svn_subr::auth::AuthBaton;

use crate::Connection;
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

/// `svn_ra_svn__session_baton_t`
pub struct SessionBaton {
    conn: Connection,
    is_tunneled: bool,
    auth_baton: AuthBaton,
    parent: Parent,
    user: String,
    /// The remote hostname
    hostname: String,
    realm_prefix: String,
    tunnel_name: Option<String>,
    tunnel_args: Vec<String>,
    config: HashMap<String, String>,
    bytes_read: u64,
    bytes_written: u64,

    useragent: String,
}

/// The session's URL state for client and server side.
///
/// This keeps track of the respective client-side and server-side "parent"
/// URLs.  It tells us whether we may have to send reparent commands to the
/// server and how to tweak path parameters when we decided to handle
/// reparent requests on the client side only.
///
/// `svn_ra_svn__parent_t`
pub struct Parent {
    /// Client-side session base URL, i.e. client's parent path.
    client_url: String,
    /// Server-side base URL, i.e. server's parent path.
    server_url: String,
    /// Relative path to add to a client-side parameter to translate it for the
    /// server-side.  I.e. the relative path from SERVER_URL to CLIENT_URL.
    path: String,
}
