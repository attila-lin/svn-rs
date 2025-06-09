mod node_kind;
pub use node_kind::NodeKind;

mod commit_info;
mod depth;
pub use depth::Depth;

mod directory_entry;
mod lock;

pub use directory_entry::DirectoryEntry;

/// A revision number.
///
/// @note Update svnxx/revision.hpp when changing this typedef.
pub type RevisionNumber = i64;

/// The maximum size of an expanded or un-expanded keyword.
pub const SVN_KEYWORD_MAX_LEN: usize = 255;
/// The most recent revision in which this file was changed.
pub const SVN_KEYWORD_REVISION_LONG: &str = "LastChangedRevision";
/// Short version of LastChangedRevision
pub const SVN_KEYWORD_REVISION_SHORT: &str = "Rev";
/// Medium version of LastChangedRevision, matching the one CVS uses.
pub const SVN_KEYWORD_REVISION_MEDIUM: &str = "Revision";
