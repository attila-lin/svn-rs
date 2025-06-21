use svn_types::RevisionNumber;

/// A revision, specified in one of @c svn_opt_revision_kind ways.
///
/// `svn_opt_revision_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptRevision(OptRevisionKind);

/// Various ways of specifying revisions.
///
/// @note
/// In contexts where local mods are relevant, the `working' kind
/// refers to the uncommitted "working" revision, which may be modified
/// with respect to its base revision.  In other contexts, `working'
/// should behave the same as `committed' or `current'.
///
/// `svn_opt_revision_kind`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptRevisionKind {
    Unspecified(RevisionNumber),
    Number(RevisionNumber),
    Date(i64),
    Committed(RevisionNumber),
    Previous(RevisionNumber),
    Base(RevisionNumber),
    Working(RevisionNumber),
    Head,
}
