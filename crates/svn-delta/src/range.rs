/// A node in the range index tree.
/// `range_index_node_t`
pub struct RangeIndexNode {
    /// 'offset' and 'limit' define the range in the source window.
    offset: usize,
    limit: usize,

    /// 'target_offset' is where that range is represented in the target.
    target_offset: usize,

    // 'left' and 'right' link the node into a splay tree.
    left: Option<Box<RangeIndexNode>>,
    right: Option<Box<RangeIndexNode>>,

    // 'prev' and 'next' link it into an ordered, doubly-linked list.
    prev: Option<Box<RangeIndexNode>>,
    next: Option<Box<RangeIndexNode>>,
}

/// A node in a list of ranges for source and target op copies.
/// `range_kind`
pub enum RangeKind {
    /// A range in the source window.
    Source,
    /// A range in the target window.
    Target,
}

///
/// `range_list_node_t`
pub struct RangeListNode {
    /// Where does the range come from?
    /// 'offset' and 'limit' always refer to the "virtual" source data
    /// for the second delta window. For a target range, the actual
    /// offset to use for generating the target op is 'target_offset';
    /// that field isn't used by source ranges.
    kind: RangeKind,

    /// The offset in the source or target window.
    offset: usize,

    /// The length of the range.
    length: usize,

    /// 'target_offset' is the start of the range in the target.
    target_offset: usize,

    // 'prev' and 'next' link the node into an ordered, doubly-linked list.
    prev: Option<Box<RangeListNode>>,
    next: Option<Box<RangeListNode>>,
}

/// `offset_index_t`
struct OffsetIndex {
    len: i32,
    offset: usize,
}
