use std::collections::HashMap;

use svn_fs::SvnFs;
use svn_types::RevisionNumber;

use crate::Repos;

pub struct EditBaton {
    // Revision properties to set for this commit.
    revprop_table: HashMap<String, String>,

    // The already-open svn repository to commit to.
    repos: &Repos,
    // URL to the root of the open repository.
    repos_url_decoded: String,

    // The name of the repository (here for convenience).
    repos_name: String,
    // The filesystem associated with the REPOS above (here for
    // convenience).
    fs: &SvnFs,
    // Location in fs where the edit will begin.
    base_path: String,

    // Does this set of interfaces 'own' the commit transaction?
    txn_owner: bool,
    // svn transaction associated with this edit (created in
    // open_root, or supplied by the public API caller).
    txn: Option<FsTxn>,

    /** Filled in when the edit is closed: **/
    // The new revision created by this commit.
    new_rev: RevisionNumber,

    // The date (according to the repository) of this commit.
    committed_date: String,

    // The author (also according to the repository) of this commit.
    committed_author: String,
}
