mod repository;
pub mod serve;
pub use repository::Repository;

mod client_info;
pub use client_info::ClientInfo;

mod args;
mod connection;

mod constant;

#[cfg(feature = "_sasl")]
pub mod cyrus_auth;

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

/// `server_baton_t`
pub struct ServerBaton {
    /// repository-specific data to use
    pub repository: Repository,
    /// client-specific data to use
    client_info: ClientInfo,

    /// Disallow write access (global flag)
    read_only: bool,
    /// Use virtual-host-based path to repo
    vhost: bool,
}
