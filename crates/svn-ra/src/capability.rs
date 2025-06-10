//! The capability
//!
//! `svn_ra.h` -> `SVN_RA_CAPABILITY`

use strum::IntoStaticStr;

/// `SVN_RA_CAPABILITY_*`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RaCapability {
    /// The capability of understanding @c svn_depth_t (e.g., the server
    //  * understands what the client means when the client describes the
    //  * depth of a working copy to the server.)
    Depth,
    /// The capability of doing the right thing with merge-tracking
    //  * information.  This capability should be reported bidirectionally,
    //  * because some repositories may want to reject clients that do not
    //  * self-report as knowing how to handle merge-tracking.
    #[strum(serialize = "mergeinfo")]
    MergeInfo,
    /// The capability of retrieving arbitrary revprops in svn_ra_get_log2
    LogRevprops,
    /// The capability of replaying a directory in the repository (partial replay).
    PartialReplay,
    /// The capability of including revision properties in a commit.
    CommitRevprops,
    /// The capability of specifying (and atomically verifying) expected
    //  * preexisting values when modifying revprops.
    AtomicRevprops,
    /// The capability to get inherited properties.
    InheritedProps,
    /// The capability of a server to walk revisions backwards in
    //  * svn_ra_get_file_revs2
    GetFileRevsReversed,
    /// The capability of a server to understand the list command.
    List,
}
