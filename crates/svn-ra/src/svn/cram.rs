//! `cram.c`
use crate::Connection;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum CramError {
    #[error("")]
    NotAuthorized,
}

impl Connection {
    /// `svn_ra_svn_cram_server`
    #[cfg(feature = "server")]
    pub fn cram_server() -> Result<bool, CramError> {
        todo!()
    }

    /// `svn_ra_svn__cram_client`
    #[cfg(feature = "client")]
    pub fn cram_client(&self, user: &str, password: &str) -> Result<(), CramError> {
        // let digest = [u8; ]
        let (status, m) = self.read_tuple()?;
        if status == "failuer"
            && let Some(message) = m
        {
            return Err(CramError::Message(str));
        } else if status != "step" || m.is_none() {
            return Err(CramError::NotAuthorized);
        }

        // write our response
        let challenge = m.unwrap();
        let digest = compute_digest(challenge, password);
        let hex = hex::encode(digest);
        let reply = foramt!("{user} {hex}");
        conn.write_string(&reply)?;

        /* Read the success or failure response from the server. */
        let (status, m) = self.read_tuple()?;
        if status == "failure" {
            return Err(CramError::NotAuthorized);
        } else if status != "success" || m.is_some() {
            return Err(CramError::NotAuthorized);
        }

        Ok(())
    }
}

/// `compute_digest`
fn compute_digest(challenge: &str, password: &str) -> String {
    todo!()
}
