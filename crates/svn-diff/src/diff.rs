//!

/// `svn_diff__type_e`
pub enum DiffType {
    Common,
    DiffModified,
    DiffLatest,
    DiffCommon,
    Conflict
}

/// An opaque type that represents a difference between either two or
/// three datasources.   This object is returned by svn_diff_diff(),
/// svn_diff_diff3() and svn_diff_diff4(), and consumed by a number of
/// other routines.
///
/// `svn_diff_t`
pub struct SvnDiff {
    next: Box<SvnDiff>,
    r#type: DiffType,

    original_start: i64,
    original_len: i64,
    modified_start: i64,
    modified_len: i64,
    latest_start: i64,
    latest_len: i64,

    resolved_diff: Box<SvnDiff>
}
