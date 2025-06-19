//! provides different editor modes
//!
mod error;
pub use error::EditorError;

pub mod debug;
pub mod depth_filter;

use std::path::Path;

use svn_types::RevisionNumber;

/// `svn_editor_t`
pub struct SvnEditor {
    baton: Option<Box<dyn std::any::Any>>,
    /// Standard cancellation function. Called before each callback.
    cancel_func: Box<dyn Fn(&SvnEditor) -> bool>,
    /* Our callback functions match that of the set-many structure, so
    just use that.  */
}

/// `default_editor.c` & `svn_delta_editor_t`
pub trait DeltaEditor {
    fn set_target_revision(&self, target_revision: RevisionNumber) -> Result<(), EditorError>;
    fn add_item(
        &mut self,
        path: &Path,
        parent_baton: (),
        conpyfrom_path: &Path,
        copyfrom_revison: RevisionNumber,
    ) -> Result<(), EditorError>;
    fn open_root(&self, base_revison: RevisionNumber) -> Result<(), EditorError>;

    fn delete_entry(
        &mut self,
        path: &Path,
        revision: RevisionNumber,
        parent_baton: (),
    ) -> Result<(), EditorError>;

    fn open_item(&self, path: &Path, parent_baton: ()) -> Result<(), EditorError>;

    fn change_prop(&self, file_baton: (), name: &str, value: String) -> Result<(), EditorError>;
}
