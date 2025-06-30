//! session

use std::{collections::HashMap, path::Path};

use svn_subr::auth::AuthBaton;
use svn_types::{NodeKind, RevisionNumber};
use url::Url;

use crate::Connection;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum RaError {
    #[error("illegal url")]
    IllegalUrl,
}

// The RA session object.

/// A repository access session.  This object is used to perform requests
/// to a repository, identified by a URL.
///
/// `svn_ra_session_t`
///
/// `libsvn_ra_svn/client.c`
pub struct SvnRaSession(Box<dyn RaSession>);

impl SvnRaSession {
    /* Ensure that RA_SESSION's session URL matches SESSION_URL,
       reparenting that session if necessary.
       Store the previous session URL in *OLD_SESSION_URL (so that if the
       reparenting is meant to be temporary, the caller can reparent the
       session back to where it was).

       If SESSION_URL is NULL, treat this as a magic value meaning "point
       the RA session to the root of the repository".

       NOTE: The typical usage pattern for this functions is:

           const char *old_session_url;
           SVN_ERR(svn_client__ensure_ra_session_url(&old_session_url,
                                                     ra_session,
                                                     new_session_url,
                                                     pool);

           [...]

           SVN_ERR(svn_ra_reparent(ra_session, old_session_url, pool));
    */
    /// `svn_client__ensure_ra_session_url`
    pub fn ensure_ra_session_url(&self, session_url: &Option<Url>) -> Result<Url, String> {
        let old_session_url = self.get_session_url()?;

        let ret = match old_session_url {
            Some(o) => Some(o),
            None => self.get_repos_root(session_url)?,
        };
        if &ret != session_url {
            // FIXME:
            // self.reparent(session_url)?;
        }

        Ok(ret.unwrap())
    }

    /// Set @a *url to the session URL -- the URL to which @a ra_session was
    /// opened or most recently reparented.
    ///
    /// @since New in 1.5.
    ///
    /// `svn_ra_get_session_url`
    pub fn get_session_url(&self) -> Result<Option<Url>, String> {
        self.0.get_session_url()
    }

    fn get_repos_root(&self, url: &Option<Url>) -> Result<Option<Url>, String> {
        let ret = self.0.get_repos_root(url)?;
        Ok(ret)
    }

    /// Change the root URL of an open @a ra_session to point to a new path in the
    /// same repository.  @a url is the new root URL.  Use @a pool for
    /// temporary allocations.
    ///
    /// If @a url has a different repository root than the current session
    /// URL, return @c SVN_ERR_RA_ILLEGAL_URL.
    ///
    /// @since New in 1.4.
    ///
    /// `svn_ra_reparent`
    pub fn reparent(&self, url: &Option<Url>) -> Result<(), RaError> {
        // Make sure the new URL is in the same repository, so that the
        // implementations don't have to do it.
        let repos_root = self.get_repos_root(url).unwrap().unwrap();
        if !svn_subr::dirent_url::is_ancestor(&repos_root, url.as_ref().unwrap()) {
            return Err(RaError::IllegalUrl);
        }

        self.0.reparent(url)?;
        Ok(())
    }

    /// * Set @a *kind to the node kind associated with @a path at @a revision.
    ///  * If @a path does not exist under @a revision, set @a *kind to
    ///  * @c svn_node_none.  @a path is relative to the @a session's parent URL.
    ///  *
    ///  * Use @a pool for memory allocation.
    ///  *
    ///  * @since New in 1.2.
    /// `svn_ra_check_path`
    pub fn check_path(
        &self,
        path: &Path,
        revision: Option<RevisionNumber>,
    ) -> Result<NodeKind, String> {
        debug_assert_eq!(path.canonicalize().unwrap().to_str(), path.to_str());
        self.0.check_path(path, revision)
    }
}

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

    /// See svn_ra_open5().
    /// All fields in SESSION, except priv, have been initialized by the
    /// time this is called.  SESSION->priv may be set by this function.
    fn open_session(corrected_url: &str, redirect_url: &str, session_url: &str) -> Self
    where
        Self: Sized;

    /// See svn_ra_get_session_url().
    fn get_session_url(&self) -> Result<Option<Url>, String>;

    /// See svn_raget_repos_root2().
    fn get_repos_root(&self, _url: &Option<Url>) -> Result<Option<Url>, String> {
        // Placeholder implementation, should be overridden
        Err("Not implemented".to_string())
    }

    /// See svn_ra_check_path().
    fn check_path(
        &self,
        path: &Path,
        revision: Option<RevisionNumber>,
    ) -> Result<NodeKind, String> {
        Err("Not implemented".to_string())
    }

    /// See svn_ra_reparent().
    /// URL is guaranteed to have what get_repos_root() returns as a prefix.
    fn reparent(&self, _url: &Option<Url>) -> Result<(), RaError> {
        // Placeholder implementation, should be overridden
        Err(RaError::IllegalUrl)
    }
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

    fn get_session_url(&self) -> Result<Option<Url>, String> {
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
