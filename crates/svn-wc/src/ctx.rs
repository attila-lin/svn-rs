use svn_subr::SvnConfig;

use crate::WcDb;

///  Context handling
///
/// `svn_wc_context_t`
#[derive(Debug)]
pub struct WcContext {
    ///  The wc_db handle for this working copy.
    db: WcDb,

    /// Close the DB when we destroy this context?
    ///
    /// This is used inside backward compat wrappers, and should only be
    /// modified by the proper create() functions.
    close_db_on_destroy: bool,
}

impl WcContext {
    /// create a new working copy context with db
    ///
    /// Just like svn_wc_context_create(), only use the provided DB to construct
    /// the context.
    ///
    /// Even though DB is not allocated from the same pool at *WC_CTX, it is
    /// expected to remain open throughout the life of *WC_CTX.
    ///
    /// `svn_wc__context_create_with_db`
    pub fn new_with_db(db: WcDb, config: SvnConfig) -> Self {
        WcContext {
            db,
            close_db_on_destroy: false,
        }
    }
}
