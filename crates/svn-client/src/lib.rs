pub mod add;
pub mod blame;
pub mod cancel;
pub mod cat;
pub mod cmdline;
pub mod ctx;
pub mod delete;
pub mod info;
pub mod layout;
pub mod status;
pub mod update;

mod error;
pub use error::Error;

use bitflags::bitflags;

bitflags! {
    /// @name Commit state flags
    /// @brief State flags for use with the #svn_client_commit_item3_t structure
    /// (see the note about the namespace for that structure, which also
    /// applies to these flags).
    /// @{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CommitItem: u32 {
        const ADD = 0x01;
        const DELETE = 0x02;
        const TEXT_MODS = 0x04;
        const PROPERTY_MODS = 0x08;
        const IS_COPY = 0x10;

        /// One of the flags for a commit item.  The node has a lock token that
        /// should be released after a successful commit and, if the node is also
        /// modified, transferred to the server as part of the commit process.
        ///
        /// @since New in 1.2.
        const LOCK_TOKEN = 0x20;

        /// One of the flags for a commit item.  The node is the 'moved here'
        /// side of a local move.  This is used to check and enforce that the
        /// other side of the move is also included in the commit.
        ///
        /// @since New in 1.8.
        const MOVED_HERE = 0x40;
    }
}
