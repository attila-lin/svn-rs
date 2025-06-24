//! `compose_delta.c`
//! Delta window composition.

use indextree::{Arena, NodeId};

use crate::TxdeltaWindow;

/// A node in the range index tree.
///
/// `range_index_node_t`
pub struct RangeIndexNode {
    offset: usize,
    limit: usize,

    target_offset: usize,
}

pub struct RangeIndex {
    area: Arena<RangeIndexNode>,
    root: Option<NodeId>,
}

/// A node in a list of ranges for source and target op copies.
/// `range_index_t`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RangeKind {
    Source,
    Target,
}

/// Mapping offsets in the target stream to txdelta ops.
///
/// `offset_index_t`
#[derive(Debug)]
pub struct OffsetIndex {
    length: usize,
    offs: Vec<usize>,
}

impl OffsetIndex {
    /// Create an index mapping target stream offsets to delta ops in
    /// WINDOW. Allocate from POOL.
    pub fn create(window: &TxdeltaWindow) -> Self {
        let mut offset = 0;

        let mut ret = Self {
            length: window.num_ops as usize,
            offs: Vec::with_capacity((window.num_ops + 1) as usize),
        };

        for i in 0..window.num_ops as usize {
            ret.offs.push(offset);
            offset += window.ops[i].length as usize;
        }
        ret.offs.push(offset);

        ret
    }

    /* Find the index of the delta op thet defines that data at OFFSET in
    NDX. HINT is an arbitrary positin within NDX and doesn't even need
    to be valid. To effectively speed up the search, use the last result
    as hint because most lookups come as a sequence of decreasing values
    for OFFSET and they concentrate on the lower end of the array. */
    fn search_offset(&self, offset: usize, hint: usize) -> usize {
        // Implementation would be similar to the C version but using Rust's
        // built-in binary search capabilities
        let mut lo = 0;
        let mut hi = self.length;

        // Use hint if valid
        if hint < hi {
            if offset < self.offs[hint] {
                hi = hint;
            } else if offset < self.offs[hint + 1] {
                return hint;
            } else {
                lo = hint + 1;
            }
        }

        // Binary search
        while lo < hi {
            let mid = (lo + hi) / 2;
            if offset < self.offs[mid] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo - 1
    }
}
