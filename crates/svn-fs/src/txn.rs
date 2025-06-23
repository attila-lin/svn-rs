use svn_types::RevisionNumber;

use crate::SvnFs;

/// `svn_fs_txn_t`
pub struct FsTxn {
    // The filesystem to which this transaction belongs
    fs: SvnFs,
    // The revision on which this transaction is based, or
    // if the transaction is not based on a
    // revision at all
    base_rev: RevisionNumber,
    /// The ID of this transaction
    id: String,
}

/// `txn_vtable_t`
trait Txn {
    type FsapData;
}

impl Txn for FsTxn {
    type FsapData = ();
}
