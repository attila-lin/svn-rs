//! `svn_ra_svn_conn_st`
use std::{collections::HashMap, net::IpAddr};

use bytes::BytesMut;
use svn_subr::Uuid;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {}

/// This structure is opaque to the server.  The client pokes at the
/// first few fields during setup and cleanup.
///
/// `svn_ra_svn_conn_st`
pub struct Connection {
    write_buf: BytesMut,
    read_buf: BytesMut,

    /* Although all reads and writes go through the svn_ra_svn__stream_t
    interface, SASL still needs direct access to the underlying socket
    for stuff like IP addresses and port numbers. */
    #[cfg(feature = "_sasl")]
    sock: Socket,
    #[cfg(feature = "_sasl")]
    encrypted: bool,

    /// repository info
    uuid: Uuid,
    repos_root: String,

    /// TX block notification target
    block_handler: BlockHandler,

    // server settings
    capabilities: HashMap<String, String>,
    compression_level: u32,
    zero_copy_limit: usize,

    /// who's on the other side of the connection
    remote_ip: IpAddr,

    /// EV2 support
    shim_callable: (),
}

type BlockHandler = Box<dyn Fn(&Connection) -> Result<(), ConnectionError>>;
