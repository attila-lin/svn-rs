/// A lock object, for client & server to share.
///  *
///  * A lock represents the exclusive right to add, delete, or modify a
///  * path.  A lock is created in a repository, wholly controlled by the
///  * repository.  A "lock-token" is the lock's UUID, and can be used to
///  * learn more about a lock's fields, and or/make use of the lock.
///  * Because a lock is immutable, a client is free to not only cache the
///  * lock-token, but the lock's fields too, for convenience.
///  *
///  * Note that the 'is_dav_comment' field is wholly ignored by every
///  * library except for mod_dav_svn.  The field isn't even marshalled
///  * over the network to the client.  Assuming lock structures are
///  * created with apr_pcalloc(), a default value of 0 is universally safe.
///  *
///  * @note in the current implementation, only files are lockable.
///
/// @from svn_types.h svn_lock_t
pub struct Lock {
    /// the path this lock applies to
    path: String,
    /// unique URI representing lock
    token: String,
    /// the username which owns the lock
    owner: String,
    /// (optional) description of lock
    comment: Option<String>,
    /// was comment made by generic DAV client?
    is_dav_comment: bool,
    /// when lock was made
    creation_date: i64,
    /// (optional) when lock will expire;
    /// If value is 0, lock will never expire.
    expiration_date: i64,
}
