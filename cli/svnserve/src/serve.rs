//! `serve.c`

use svn_ra::Connection;

use crate::{AccessType, ServerBaton};

/// `commit_callback_baton_t`
pub struct CommitCallbackBaton {
    new_rev: RevisionNumber,
    date: Vec<String>,
    author: String,
    post_commit_error: Option<String>,
}

/* Authenticate, once the client has chosen a mechanism and possibly
 * sent an initial mechanism token.  On success, set *success to true
 * and b->user to the authenticated username (or NULL for anonymous).
 * On authentication failure, report failure to the client and set
 * *success to FALSE.  On communications failure, return an error.
 * If NEEDS_USERNAME is TRUE, don't allow anonymous authentication. */
pub fn auth(
    conn: &Connection,
    b: &ServerBaton,
    mech: &str,
    mecharg: Option<&str>,
    required: AccessType,
) -> Result<bool, Error> {
    let mut success = false;

    if b.repository.auth_access >= required
        && let Some(tunnel_user) = &b.client_info.tunnel_user
        && mech == "EXTERNAL"
    {
        if let Some(m) = mecharg
            && m != tunnel_user
        {
            conn.write_tuple(&["w(c)", "failure", "Requested username does not match"])?;
            return Ok(success);
        }

        b.client_info.user = tunnel_user.clone();
        conn.write_tuple(&["w()", "success"])?;
        success = true;
        return Ok(success);
    }

    if b.request.auth_access >= required && mech == "ANONYMOUS" && !needs_username {
        conn.write_tuple(&["w()", "success"])?;
        success = true;
        return Ok(success);
    }

    if b.repository.auth_access >= required
        && let Some(pwdb) = &b.repository.pwdb
        && mech == "CRAM-MD5"
    {
        let success = conn.cram_server(pwdb, user)?;
        b.client_info.user = user.to_string();
        Ok(success)
    }

    Ok(success)
}
