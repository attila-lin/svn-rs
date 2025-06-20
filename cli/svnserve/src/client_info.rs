use svn_ra::Connection;

/// `client_info_t`
pub struct ClientInfo {
    /// Authenticated username of the user
    pub user: String,
    /// IP of the client that contacted the server
    pub remote_host: String,
    /// Username for authz ('user' + 'username_case')
    pub authz_user: String,

    /// Tunneled through login agent
    pub tunnel: bool,
    /// Allow EXTERNAL to authenticate as this
    pub tunnel_user: Option<String>,
}

impl ClientInfo {
    pub fn from_connection(conn: Connection, params: &ServerParams) -> Self {}
}
