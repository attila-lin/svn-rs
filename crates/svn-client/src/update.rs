//! wrapper around wc update functionality

use std::{collections::HashMap, path::Path};

use svn_ra::session::SvnRaSession;
use svn_subr::io::Dirent;
use svn_types::{NodeKind, RevisionNumber};
use svn_wc::conflict::ConflictDescription;
use svn_wc::conflict::ConflictResult;
use url::Url;

use crate::Error;

/// Baton for svn_client__dirent_fetcher
pub struct DirentFetcherBaton {
    ra_session: SvnRaSession,
    target_revision: RevisionNumber,
    anchor_url: Url,
}

/// Implements svn_wc_dirents_func_t for update and switch handling. Assumes
/// a struct svn_client__dirent_fetcher_baton_t * baton
///
/// `svn_client__dirent_fetcher`
pub fn dirent_fetcher(
    dfb: DirentFetcherBaton,
    dirents: HashMap<String, Dirent>,
    repos_root_url: &Url,
    repos_relpath: &str,
) -> Result<(), Error> {
    let url = svn_subr::path::add_component(repos_root_url, repos_relpath)?;
    let session_relpath = if !svn_subr::dirent_url::is_ancestor(&dfb.anchor_url, &url) {
        dfb.ra_session.ensure_ra_session_url(url)?;
        ""
    } else {
        dfb.ra_session.get_path_relative_to_session(url)?
    };

    // Is session_relpath still a directory?
    let kind = dfb
        .ra_session
        .check_path(session_relpath, dfb.target_revision)?;

    let dirents = if kind == NodeKind::Directory {
        dfb.ra_session.get_dir()?
    } else {
        None
    };

    if let Some(o) = &old_url {
        dfb.ra_session.reparent(o)?;
    }

    Ok(())
}

/// Set *CLEAN_CHECKOUT to FALSE only if LOCAL_ABSPATH is a non-empty
/// folder. ANCHOR_ABSPATH is the w/c root and LOCAL_ABSPATH will still
/// be considered empty, if it is equal to ANCHOR_ABSPATH and only
/// contains the admin sub-folder.
/// If the w/c folder already exists but cannot be opened, we return
/// "unclean" - just in case. Most likely, the caller will have to bail
/// out later due to the same error we got here.
///
/// `is_empty_wc`
pub fn is_empty_wc(local_abspath: &Path, anchor_abspath: &Path) -> bool {
    let mut clean_checkout = true;

    /* open directory. If it does not exist, yet, a clean one will
    be created by the caller. */
    let dir = match std::fs::read_dir(local_abspath) {
        Ok(dir) => dir,
        Err(_) => {
            return false; // Directory does not exist, so it is clean
        }
    };

    for entry in dir {
        // Ignore entries for this dir and its parent, robustly.
        // (APR promises that they'll come first, so technically
        // this guard could be moved outside the loop.  But Ryan Bloom
        // says he doesn't believe it, and I believe him.
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue, // Ignore errors reading entries
        };
        if entry.is_dir() {
            if !svn_wc::Adm::is_adm_dir(entry.path()) || local_abspath != anchor_abspath {
                clean_checkout = false;
                break; // Found a non-admin entry, so not empty
            }
        }
    }

    clean_checkout
}

/// A conflict callback that simply records the conflicted path in BATON.
///
/// Implements `svn_wc_conflict_resolve_func2_t`
pub fn record_conflict(
    description: &svn_wc::WcConflictDescription,
    conflict_paths: &HashSet<String>,
) -> Result<ConflictResult, Error> {
    let v = conflict_paths.get(description.local_abspath).unwrap();
    conflict_paths.insert(v);
    let res = ConflictResult::create(svn_wc::conflict::ConflictChoice::Postpone, None);
    Ok(res)
}
