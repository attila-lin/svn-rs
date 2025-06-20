//! blame.c:  return blame messages

use std::{collections::HashMap, path::PathBuf};

use svn_types::RevisionNumber;

/// The metadata associated with a particular revision.
/// `rev`
pub struct Rev {
    /// The revision number.
    revision: RevisionNumber,
    /// The revison properties.
    rev_props: HashMap<String, String>,
    /// The absolute repository path.
    /// used for merge reporting.
    path: PathBuf,
}

/// One chunk of blame
/// `blame`
pub struct Blame {
    /// the responsible revision
    rev: Rev,
    /// the starting diff-token (line)
    start: usize,
}

/// A chain of blame chunks
pub struct BlameChain {
    /// linked list of blame chunks
    blames: Vec<Blame>,
    /// linked list of free blame chunks
    avail: Vec<Blame>,
}
