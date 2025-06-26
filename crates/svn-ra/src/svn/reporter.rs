use svn_delta::editor::DeltaEditor;
use svn_types::{Depth, RevisionNumber, SvnError};

use super::Connection;
use super::SessionBaton;
use crate::reporter::Reporter;

/// `ra_svn_reporter_baton_t`
pub struct ReporterBaton {
    session: SessionBaton,
    conn: Connection,
    editor: Box<dyn DeltaEditor>,
}

impl Reporter for ReporterBaton {
    fn set_path(
        &mut self,
        _path: &str,
        _rev: Option<RevisionNumber>,
        _depth: Depth,
        _start_empty: bool,
        _lock_token: Option<&str>,
    ) -> Result<(), SvnError> {
        let _conn = &mut self.conn;
        // conn
        todo!()
    }

    fn delete_path(&mut self, _path: &str) -> Result<(), SvnError> {
        todo!()
    }

    fn link_path(
        &mut self,
        _path: &str,
        _url: &str,
        _revision: Option<i64>,
        _depth: Depth,
        _start_empty: bool,
        _lock_token: Option<&str>,
    ) -> Result<(), SvnError> {
        todo!()
    }

    fn finish_report(&mut self) -> Result<(), svn_types::SvnError> {
        todo!()
    }

    fn abort_report(&mut self) -> Result<(), SvnError> {
        todo!()
    }
}
