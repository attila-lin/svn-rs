use svn_delta::editor::DeltaEditor;
use svn_types::RevisionNumber;

/// Edit baton
pub struct EditBaton {
    wrapped_editor: Box<dyn DeltaEditor>,
    to_url: String,
    source_prop_encoding: String,

    called_open_root: bool,
    got_textdeltas: bool,
    base_revision: Option<RevisionNumber>,
    quiet: bool,
    /// Did we tweak the mergeinfo?
    mergeinfo_tweaked: bool,
    strip_mergeinfo: bool,
    migrae_svnmerge: bool,
    mergeinfo_sripped: bool,
    svnmerge_migrated: bool,
    svnmerge_blocked: bool,
    /// Where to count normalizations
    normalized_node_props_counter: i32,
}
