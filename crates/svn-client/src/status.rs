use std::path::PathBuf;

use svn_types::NodeKind;
use svn_wc::status::StatusKind;
use url::Url;
use uuid::Uuid;

/// * Structure for holding the "status" of a working copy item.
///  *
///  * @note Fields may be added to the end of this structure in future
///  * versions.  Therefore, to preserve binary compatibility, users
///  * should not directly allocate structures of this type.
///  *
///  * @since New in 1.7.
/// `svn_client_status_t`
#[derive(Debug, Clone)]
pub struct Status {
    /// The kind of node as recorded in the working copy.
    kind: NodeKind,
    /// The absolute path to the node
    local_abspath: PathBuf,
    /// The actual size of the working file on disk, or `None` if the node is unknown (or if the item isn't a file at all)
    filesize: Option<u64>,
    /// Set to `true` if the node is the victim of some kind of conflict
    conflicted: bool,

    /// The status of the node, basedon the restructuring changes and if the node has no restructuring changes the text and prop status.
    node_status: StatusKind,

    /** The status of the text of the node, not including restructuring changes.
     * Valid values are: svn_wc_status_none, svn_wc_status_normal,
     * svn_wc_status_modified and svn_wc_status_conflicted. */
    text_status: StatusKind,

    /** The status of the node's properties.
     * Valid values are: svn_wc_status_none, svn_wc_status_normal,
     * svn_wc_status_modified and svn_wc_status_conflicted. */
    prop_status: StatusKind,

    wc_is_locked: bool,

    copied: bool,

    repos_root_url: Url,
    repos_uuid: Uuid,
}

impl Status {
    /// Create a new `Status` instance with default values.
    ///
    /// `svn_client_status6`
    pub fn new(
        kind: NodeKind,
        local_abspath: PathBuf,
        repos_root_url: Url,
        repos_uuid: Uuid,
    ) -> Self {
        Self {
            kind,
            local_abspath,
            filesize: None,
            conflicted: false,
            node_status: StatusKind::None,
            text_status: StatusKind::None,
            prop_status: StatusKind::None,
            wc_is_locked: false,
            copied: false,
            repos_root_url,
            repos_uuid,
        }
    }

    // Additional methods can be added here as needed
}
