use crate::RevisionNumber;

/**
 * Mergeinfo representing a merge of a range of revisions.
 *
 * @since New in 1.5
 */
/// `svn_merge_range_t`
pub struct MergeRange {
    /**
     * If the 'start' field is less than the 'end' field then 'start' is
     * exclusive and 'end' inclusive of the range described.  This is termed
     * a forward merge range.  If 'start' is greater than 'end' then the
     * opposite is true.  This is termed a reverse merge range.  If 'start'
     * equals 'end' the meaning of the range is not defined.
     */
    start: RevisionNumber,
    end: RevisionNumber,

    /**
     * Whether this merge range should be inherited by treewise
     * descendants of the path to which the range applies. */
    inheritable: bool,
}
