//! `compose_delta.c`
//! Delta window composition.

use crate::TxdeltaWindow;

/// A node in the range index tree.
///
/// `range_index_node_t`
pub struct RangeIndexNode {
    offset: usize,
    limit: usize,

    target_oofset: usize,

    left: Option<Box<RangeIndexNode>>,
    right: Option<Box<RangeIndexNode>>,

    prev: Option<Box<RangeIndexNode>>,
    next: Option<Box<RangeIndexNode>>,
}

/// A node in a list of ranges for source and target op copies.
/// `range_index_t`
pub enum RangeKind {
    Source,
    Target,
}

/// Mapping offsets in the target stream to txdelta ops.
/// `offset_index_t`
pub struct OffsetIndex {
    length: i32,
    offs: Vec<i32>,
}

impl OffsetIndex {
    /* Create an index mapping target stream offsets to delta ops in
    WINDOW. Allocate from POOL. */
    pub fn create(window: &TxdeltaWindow) -> Self {
        let mut offset = 0;

        let mut ret = Self {
            length: window.num_ops,
            offs: Vec::with_capacity((window.num_ops + 1) as usize),
        };

        for i in 0..window.num_ops as usize {
            ret.offs.push(offset);
            offset += window.ops[i].length as i32;
        }
        ret.offs.push(offset);

        ret
    }
}
