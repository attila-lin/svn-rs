mod fs;
mod fsx;
mod root;

use svn_types::RevisionNumber;

/// `fs_vtable_t`
pub trait FsTrait {
    fn youngest_rev(&self) -> RevisionNumber;
    fn refresh_revision_prop(&self) -> Result<(), ()>;

    fn revision_prop() -> Result<(), ()>;
}
