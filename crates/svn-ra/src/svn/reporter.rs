/// `ra_svn_reporter_baton_t`
pub struct ReporterBaton {
    session: SessionBaton,
    conn: Connection,
    editor: Box<dyn DeltaEditor>,
}

impl ReporterBaton {
    pub fn set_path(&mut self, path: &str, rev: RevisionNumber) -> Result<(), ReportError> {
        let conn = &mut self.conn;
        // conn
        todo!()
    }
}
