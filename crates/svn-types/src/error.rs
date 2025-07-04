//! `svn_error_codes.h`

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SvnError {
    #[error(transparent)]
    Fs(#[from] SvnFsError),
    #[error(transparent)]
    Client(#[from] SvnClientError),
    #[error(transparent)]
    Misc(#[from] SvnMiscError),
    #[error(transparent)]
    Ra(#[from] SvnRaError),
    #[error(transparent)]
    Node(#[from] SvnNodeError),
    #[error(transparent)]
    Wc(#[from] SvnWcError),
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SvnFsError {
    #[error("General filesystem error")]
    General,

    #[error("Error closing filesystem")]
    Cleanup,

    #[error("Filesystem is already open")]
    AlreadyOpen,

    #[error("Filesystem is not open")]
    NotOpen,

    #[error("Filesystem is coppupted")]
    Coppupt,

    #[error("Invalid filesystem path syntax")]
    PathSyntax,

    #[error("Invalid filesystem revision number")]
    NoSuchRevision,

    #[error("Invalid filesystem transaction name")]
    NoSuchTransaction,

    #[error("Filesystem directory has no such entry")]
    NoSuchEntry,

    #[error("Filesystem has no such represenetation")]
    NoSuchRepresentation,

    #[error("Filesystem has no such string")]
    NoSuchString,

    #[error("Filesystem has no such copy")]
    NoSuchCopy,

    #[error("The specified transaction is not mutable")]
    TransactionNotMutable,

    #[error("Filesystem has no item")]
    NotFound,

    #[error("Filesystem has no such node-rev-id")]
    IdNotFound,

    #[error("String does not represent a node or node-rev-id")]
    NotId,

    #[error("Name does not refer to a filesystem directory")]
    NotDirectory,

    #[error("Name does not refer to a filesystem file")]
    NotFile,

    #[error("Name is not a single path component")]
    NotSinglePathComponent,

    #[error("Attempt to change immutable filesystem node")]
    NotMutable,

    #[error("Item already exists in filesystem")]
    AlreadyExists,
    #[error("Attempt to remove or recreate fs root dir")]
    RootDir,

    #[error("Object is not a transaction root")]
    NotTxnRoot,

    #[error("Object is not a revision root")]
    NotRevisionRoot,

    #[error("Merge conflict during commit")]
    Conflict,

    #[error("A representation vanished or changed between reads")]
    RepChanged,

    #[error("Tried to change an immutable representation")]
    RepNotMutable,

    #[error("Malformed skeleton data")]
    MalformedSkeleton,

    #[error("Transaction is out of date")]
    TxnOutOfDate,
    #[error("Berkeley DB error")]
    BerkeleyDb,
    #[error("Berkeley DB deadlock error")]
    BerkeleyDbDeadlock,
    #[error("Transaction is dead")]
    TransactionDead,

    #[error("Transaction is not dead")]
    TransactionNotDead,

    #[error("Unknown FS type")]
    UnknownFsType,

    #[error("No user associated with filesystem")]
    NoUser,

    #[error("Path is already locked")]
    PathAleadyLocked,

    #[error("Path is not locked")]
    PathNotLocked,

    #[error("Lock token is incorrect")]
    BadLockToken,

    #[error("No lock token provided")]
    NoLockToken,

    #[error("Username does not match lock owner")]
    LockOwnerMismatch,
    #[error("Filesystem has no such lock")]
    NoSuchLock,

    #[error("Lock has expired")]
    LockExpired,
    #[error("Item is out of date")]
    OutOfDate,
    #[error("Unsupported FS format")]
    UnsupportedFormat,
    #[error("Representation is being written")]
    RepBeingWritten,
    #[error("The generated transaction name is too long")]
    TxnNameTooLong,
    #[error("Filesystem has no such node origin record")]
    NoSuchNodeOrigin,
    #[error("Filesystem upgrade is not supported")]
    UnsupportedUpgrade,
    #[error("Filesystem has no such checksum-representation index record")]
    NoSuchChecksumRep,

    #[error("Property value in filesystem differs from the provided base value")]
    PropBasevalueMismatch,
    #[error("The filesystem editor completion process was not followed")]
    IncorrectEditorCompletion,
    #[error("A packed revprop could not be read")]
    PackedRevpropReadFailure,
    #[error("Could not initialize the revprop caching infrastructure.")]
    RevpropCacheInitFailure,

    #[error("Malformed transaction ID string.")]
    MalformedTxnId,
    #[error("Corrupt index file.")]
    IndexCorruption,

    #[error("Revision not covered by index.")]
    IndexRevsion,

    #[error("Item index too large for this revision.")]
    IndexOverflow,

    #[error("Container index out of range.")]
    ContainerIndex,

    #[error("Index files are inconsistent.")]
    IndexInconsistent,
    #[error("Lock operation failed")]
    LockOperationFailed,

    #[error("Unsupported FS type")]
    UnsupportedFsType,
    #[error("Container capacity exceeded.")]
    ContainerSize,
    #[error("Malformed node revision ID string.")]
    MalformedNodeRevId,
    #[error("Invalid generation number data.")]
    InvalidGeneration,
    #[error("Revprop manifest corrupt.")]
    CorruptRevpropManifest,
    #[error("Property list is corrupt.")]
    CorruptPropList,
    #[error("Content checksums supposedly match but content does not.")]
    AmbiguousChecksumRep,
    #[error("Unrecognized filesystem I/O control code")]
    UnrecognizedIoctlCode,
    #[error("Rep-sharing is not allowed.")]
    RepSharingNotAllowed,
    #[error("Rep-sharing is not supported.")]
    RepSharingNotSupported,
}

/* generic RA errors */
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SvnRaError {
    #[error("Bad URL passed to RA layer")]
    IllegalUrl,

    #[error("Authentication failed")]
    NotAuthenticated,

    #[error("Unknown authentication method")]
    UnknownAuth,

    #[error("Repository access method not implemented")]
    NotImplemented,

    #[error("Item is out of date")]
    OutOfDate,

    #[error("Repository has no UUID")]
    NoReposUUID,

    #[error("Unsupported RA plugin ABI version")]
    UnsupportedABIVersion,

    #[error("Path is not locked")]
    NotLocked,

    #[error("Server can only replay from the root of a repository")]
    PartialReplayNotSupported,

    #[error("Repository UUID does not match expected UUID")]
    UUIDMismatch,

    #[error("Repository root URL does not match expected root URL")]
    RootUrlMismatch,
    #[error("Session URL does not match expected session URL")]
    SessionUrlMismatch,
    #[error("Can't create tunnel")]
    CannotCreateTunnel,
    #[error("Can't create session")]
    CannotCreateSession,
}

/// Node errors
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SvnNodeError {
    #[error("Unknown svn_node_kind")]
    UnknownKind,
    #[error("Unexpected node kind found")]
    UnexpectedKind,
}

/// misc errors
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SvnMiscError {
    #[error("Tried a versioning operation on an unversioned resource")]
    UnversionedResource,
}

/// client errors
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SvnClientError {
    #[error("Attempting restricted operation for modified resource")]
    Modified,

    #[error("Bad property name")]
    PropertyName,
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SvnWcError {
    #[error("todo")]
    CannotDeleteFileExternal,
}
