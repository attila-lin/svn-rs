/// A list of svn_ra_svn__item_t objects.
///
/// `svn_ra_svn__list_t`
pub struct SvnItemList {
    /// List contents (array).  May be NULL if NELTS is 0.
    items: Vec<SvnItem>,
    /// Number of elements in ITEMS.
    nelts: usize,
}

/// Memory representation of an on-the-wire data item.
/// Data types defined by the svn:// protocol.
///
/// `svn_ra_svn__item_t` & `svn_ra_svn_item_kind_t`
pub enum SvnItem {
    Number(u64),
    String(String),
    Word(String),
    List(SvnItemList),
}
