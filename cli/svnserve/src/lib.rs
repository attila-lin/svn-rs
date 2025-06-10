mod repository;
pub use repository::Repository;

mod client_info;
pub use client_info::ClientInfo;

mod args;
pub use args::AppArgs;

/// The strategy for handling incoming connections.  Some of these may be
/// unavailable due to platform limitations.
pub enum ConnectionHandlingMode {
    /// Create a process per connection
    Fork,
    /// Create a thread per connection
    Thread,
    /// One connection at a time in this process
    Single,
}

/// The mode in which to run svnserve
pub enum RunMode {
    Unspecified,
    Inetd,
    Daemon,
    Tunnel,
    ListenOnce,
    Service,
}

/// `username_case_type`
pub enum UsernameCaseType {
    Upper,
    Lower,
    Asis,
}

/// `authn_type`
pub enum AuthnType {
    Unauthenticated,
    Authenticated,
}

/// `access_type`
pub enum AccessType {
    No,
    Read,
    Write,
}
