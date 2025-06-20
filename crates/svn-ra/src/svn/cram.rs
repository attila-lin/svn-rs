//! `cram.c`
use crate::Connection;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum CramError {}

impl Connection {
    /// `svn_ra_svn_cram_server`
    pub fn cram_server() -> Result<bool, CramError> {
        todo!()
    }
}
