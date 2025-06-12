/// Action codes for text delta instructions.
///
/// `svn_delta_action`
pub enum Action {
    /// Append the @a length bytes at @a offset in the source view to the
    /// target.
    ///
    /// It must be the case that 0 <= @a offset < @a offset +
    /// @a length <= size of source view.
    Source,
    /// Append the @a length bytes at @a offset in the target view, to the
    //      * target.
    //      *
    //      * It must be the case that 0 <= @a offset < current position in the
    //      * target view.
    //      *
    //      * However!  @a offset + @a length may be *beyond* the end of the existing
    //      * target data.  "Where the heck does the text come from, then?"
    //      * If you start at @a offset, and append @a length bytes one at a time,
    //      * it'll work out --- you're adding new bytes to the end at the
    //      * same rate you're reading them from the middle.  Thus, if your
    //      * current target text is "abcdefgh", and you get an #svn_txdelta_target
    //      * instruction whose @a offset is 6 and whose @a length is 7,
    //      * the resulting string is "abcdefghghghghg".  This trick is actually
    //      * useful in encoding long runs of consecutive characters, long runs
    //      * of CR/LF pairs, etc.
    Target,
    /// Append the @a length bytes at @a offset in the window's @a new string
    //      * to the target.
    //      *
    //      * It must be the case that 0 <= @a offset < @a offset +
    //      * @a length <= length of @a new.  Windows MUST use new data in ascending
    //      * order with no overlap at the moment; svn_txdelta_to_svndiff()
    //      * depends on this.
    New,
}

/// A single text delta instruction.
/// `svn_txdelta_op_t`
pub struct TxDeltaOp {
    /// Action code of delta instruction
    pub action: Action,
    /// Offset of delta, see #svn_delta_action for more details.
    pub offset: usize,
    /// Number of bytes of delta, see #svn_delta_action for more details.
    pub length: usize,
}
