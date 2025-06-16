mod dag;

mod data;
pub use data::FsFsData;

mod caching;
mod fs;
mod vtable;

use std::path::PathBuf;

#[derive(Debug)]
pub struct FsFsBackend {
    path: PathBuf,
    data: Option<FsFsData>,
}

impl FsFsBackend {
    pub fn new(path: PathBuf) -> Self {
        Self { path, data: None }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn data(&self) -> Option<&FsFsData> {
        self.data.as_ref()
    }

    fn _data(&self) -> &FsFsData {
        self.data.as_ref().expect("FsFsData is not set")
    }

    fn _data_mut(&mut self) -> &mut FsFsData {
        self.data.as_mut().expect("FsFsData is not set")
    }

    pub fn set_data(&mut self, data: FsFsData) {
        self.data = Some(data);
    }
}

/// The format number of this filesystem.
/// This is independent of the repository format number, and
/// independent of any other FS back ends.
///
/// Note: If you bump this, please update the switch statement in
/// svn_fs_fs__create() as well.
const FORMAT_NUMBER: u32 = 8;

/// String with a decimal representation of the FSFS format shard size.
/// Zero ("0") means that a repository with linear layout should be created.
///
/// This option will only be used during the creation of new repositories
/// and is otherwise ignored.
///
/// @since New in 1.9.
const SVN_FS_CONFIG_FSFS_SHARD_SIZE: &str = "fsfs-shard-size";

/// Enable / disable the FSFS format 7 logical addressing feature for a
/// newly created repository.
///
/// This option will only be used during the creation of new repositories
/// and is otherwise ignored.
const SVN_FS_CONFIG_FSFS_LOG_ADDRESSING: &str = "fsfs-log-addressing";
