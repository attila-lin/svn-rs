use strum::EnumString;
use strum::IntoStaticStr;

/// `svn_depth_t`
///
/// The concept of depth for directories.
//  *
//  * @note This is similar to, but not exactly the same as, the WebDAV
//  * and LDAP concepts of depth.
#[derive(Debug, PartialEq, EnumString, IntoStaticStr)]
pub enum Depth {
    /// Depth undetermined or ignored.  In some contexts, this means the
    //       client should choose an appropriate default depth.  The server
    //       will generally treat it as #svn_depth_infinity.
    Unknown,

    /// Exclude (i.e., don't descend into) directory D.
    //       @note In Subversion 1.5, svn_depth_exclude is *not* supported
    //       anywhere in the client-side (libsvn_wc/libsvn_client/etc) code;
    //       it is only supported as an argument to set_path functions in the
    //       ra and repos reporters.  (This will enable future versions of
    //       Subversion to run updates, etc, against 1.5 servers with proper
    //       svn_depth_exclude behavior, once we get a chance to implement
    //       client-side support for svn_depth_exclude.)
    Exclude,

    /// Just the named directory D, no entries.  Updates will not pull in
    //       any files or subdirectories not already present.
    Empty,

    /// D + its file children, but not subdirs.  Updates will pull in any
    //       files not already present, but not subdirectories.
    Files,

    /// D + immediate children (D and its entries).  Updates will pull in
    //       any files or subdirectories not already present; those
    //       subdirectories' this_dir entries will have depth-empty.
    Immediates,

    /// D + all descendants (full recursion from D).  Updates will pull
    //       in any files or subdirectories not already present; those
    //       subdirectories' this_dir entries will have depth-infinity.
    //       Equivalent to the pre-1.5 default update behavior.
    Infinity,
}
