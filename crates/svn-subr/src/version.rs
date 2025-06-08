//! port `subversion/include/svn_version.h`
//!

/// Version information. Each library contains a function called
//  * svn_<i>libname</i>_version() that returns a pointer to a statically
//  * allocated object of this type.
pub struct Version {
    /// Major version number
    major: i32,
    /// Minor version number
    minor: i32,
    /// Patch version number
    patch: i32,
    /// The version tag (#SVN_VER_NUMTAG). Must always point to a
    /// statically allocated string.
    tag: String,
}

impl Version {
    /// Major version number.
    /// *
    /// * Modify when incompatible changes are made to published interfaces.
    const SVN_VER_MAJOR: i32 = 1;

    /// Minor version number.
    //  *
    //  * Modify when new functionality is added or new interfaces are
    //  * defined, but all changes are backward compatible.
    const SVN_VER_MINOR: i32 = 15;

    /// Patch number.
    //  *
    //  * Modify for every released patch.
    const SVN_VER_PATCH: i32 = 0;

    /// Version tag: a string describing the version.
    //  *
    //  * This tag remains " (under development)" in the repository so that we can
    //  * always see from "svn --version" that the software has been built
    //  * from the repository rather than a "blessed" distribution.
    //  *
    //  * When rolling a tarball, we automatically replace this text with " (r1234)"
    //  * (where 1234 is the last revision on the branch prior to the release)
    //  * for final releases; in prereleases, it becomes " (Alpha 1)",
    //  * " (Beta 1)", etc., as appropriate.
    //  *
    //  * Always change this at the same time as SVN_VER_NUMTAG.
    const SVN_VER_TAG: &str = " (under development)";

    /// Revision number: The repository revision number of this release.
    //  *
    //  * This constant is used to generate the build number part of the Windows
    //  * file version. Its value remains 0 in the repository except in release
    //  * tags where it is the revision from which the tag was created.
    const SVN_VER_REVISION: i32 = 0;

    /// return version
    pub fn current() -> Self {
        todo!()
    }
}
