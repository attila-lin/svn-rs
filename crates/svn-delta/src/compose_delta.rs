//! `compose_delta.c`
//! Delta window composition.

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
