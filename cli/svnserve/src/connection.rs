use socket2::Socket;

/// This structure contains all data that describes a client / server
/// connection.  Their lifetime is separated from the thread-local
/// serving pools.
///
/// `connection_t`
#[derive(Debug)]
pub struct Connection {
    sock: Socket,
}
