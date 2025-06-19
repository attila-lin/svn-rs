//! `svn_txdelta_window_t`

use crate::action::Action;

/** An #svn_txdelta_window_t object describes how to reconstruct a
 * contiguous section of the target string (the "target view") using a
 * specified contiguous region of the source string (the "source
 * view").  It contains a series of instructions which assemble the
 * new target string text by pulling together substrings from:
 *
 *   - the source view,
 *
 *   - the previously constructed portion of the target view,
 *
 *   - a string of new data contained within the window structure
 *
 * The source view must always slide forward from one window to the
 * next; that is, neither the beginning nor the end of the source view
 * may move to the left as we read from a window stream.  This
 * property allows us to apply deltas to non-seekable source streams
 * without making a full copy of the source stream.
 */
/// `svn_txdelta_window_t`
pub struct TxdeltaWindow {
    /** The offset of the source view for this window.  */
    sview_offset: usize,

    /** The length of the source view for this window.  */
    sview_length: usize,
    /** The length of the target view for this window, i.e. the number of
     * bytes which will be reconstructed by the instruction stream.  */
    tview_length: usize,
    /** The number of instructions in this window.  */
    pub num_ops: i32,
    /** The number of svn_txdelta_source instructions in this window. If
     * this number is 0, we don't need to read the source in order to
     * reconstruct the target view.
     */
    src_ops: i32,
    /** The instructions for this window.  */
    pub ops: Vec<TxdeltaOp>,
    /** New data, for use by any `svn_txdelta_new' instructions.  */
    new_data: String,
}
/** A single text delta instruction.  */
pub struct TxdeltaOp {
    /** Action code of delta instruction */
    action_code: Action,
    /** Offset of delta, see #svn_delta_action for more details. */
    offset: usize,
    /** Number of bytes of delta, see #svn_delta_action for more details. */
    pub length: usize,
}
