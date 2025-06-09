use svn_types::Depth;
use svn_types::RevisionNumber;

/// `svn_wc_info_t`
///
/// This struct contains information about a working copy node.
//  *
//  * @note Fields may be added to the end of this structure in future
//  * versions.  Therefore, users shouldn't allocate structures of this
//  * type, to preserve binary compatibility.
#[derive(Debug, Clone)]
pub struct WcInfo {
    /// The schedule of this item
    /// FIXME: Do we still need schedule?
    pub schedule: Option<String>,
    /// If copied, the URL from which the copy was made, else @c NULL.
    pub copyfrom_url: Option<String>,
    /// If copied, the revision from which the copy was made,
    //    * else #SVN_INVALID_REVNUM.
    copyfrom_rev: Option<RevisionNumber>,

    /// A changelist the item is in, @c NULL if this node is not in a
    //    * changelist.
    changelist: Option<String>,

    ///  The depth of the item, see [`Depth`]
    depth: Depth,

    ///  The size of the file after being translated into its local
    //    * representation, or #SVN_INVALID_FILESIZE if unknown.
    //    * Not applicable for directories.
    recorded_size: i64,

    ///  The time at which the file had the recorded size recorded_size and was
    //    * considered unmodified.
    recorded_time: i64,

    /// Array of const svn_wc_conflict_description2_t * which contains info
    //    * on any conflict of which this node is a victim. Otherwise, NULL.
    conflicts: Option<Vec<String>>,

    /// The local absolute path of the working copy root.
    wcroot_abspath: String,

    /// The path the node was moved from, if it was moved here. Else NULL.
    moved_from_abspath: Option<String>,

    /// The path the node was moved to, if it was moved away. Else NULL.
    moved_to_abspath: Option<String>,

    ///  The format of the working copy.
    wc_format: i32,

    ///  Whether pristine content is stored locally or is being fetched on-demand.
    store_pristine: bool,
}
