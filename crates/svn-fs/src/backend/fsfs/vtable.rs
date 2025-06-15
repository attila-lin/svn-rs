use super::super::BackendError;
use super::super::FsInstance;
use super::FsFsBackend;

impl FsInstance for FsFsBackend {
    fn youngest_rev(&self) -> svn_types::RevisionNumber {
        todo!()
    }

    fn refresh_revision_prop(&self) -> Result<(), BackendError> {
        todo!()
    }

    fn revision_prop(&self) -> Result<(), BackendError> {
        todo!()
    }
}
