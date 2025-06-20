//! `client.c`

/// `ra_svn_reporter_baton_t`
pub struct ReporterBaton {
    conn: SvnConection,
    editor: SvnDeltaEditor,
}

impl ReporterBaton {
    pub fn set_path(&mut self, path: &str, rev: RevisionNumber) -> Result<(), ReportError> {
        let conn = &mut self.conn;
        // conn
        todo!()
    }
}
