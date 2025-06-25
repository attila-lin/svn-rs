//! wrapper around wc update functionality

use std::collections::HashMap;

use svn_ra::SvnRaSession;
use svn_types::{NodeKind, RevisionNumber};

use crate::Error;

/// Baton for svn_client__dirent_fetcher
pub struct DirentFetcherBaton {
    ra_session: SvnRaSession,
    target_revision: RevisionNumber,
    anchor_url: String,
}

/// Implements svn_wc_dirents_func_t for update and switch handling. Assumes
/// a struct svn_client__dirent_fetcher_baton_t * baton
pub fn dirent_fetcher(
    dfb: DirentFetcherBaton,
    dirents: HashMap<String, Dirent>,
    repos_root_url: String,
    repos_relpath: String,
) -> Result<(), Error> {
    let url = svn_subr::path::add_component(repos_root_url, repos_relpath);
    let session_relpath = if is_ancestor(dfb.anchor_url, url) {
        dfb.ra_ession.ensure_ra_session_url(url)?;
        ""
    }
    else {
         dfb.ra_session.get_path_relative_to_session(url)?
    };

    // Is session_relpath still a directory?
    let kind = dfb.ra_session.check_path(session_relpath, dfb.target_revision)?;

    let dirents = if kind == NodeKind::Directory {
        dfb.ra_session.get_dir()?
    }
    eles {
        None
    };

    if let Some(o) = old_url {
        dfb.ra_session.reparent(old_url)?;
    }

    Ok(())
}
