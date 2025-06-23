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

    fn data_mut(&mut self) -> &mut Box<dyn std::any::Any> {
        // self.data.as_mut().expect("Data should be initialized")
        todo!()
    }
}
