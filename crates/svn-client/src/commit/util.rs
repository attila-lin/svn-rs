//! commit_util.c
//!

use std::collections::HashMap;

use svn_types::{SvnError, error::SvnClientError};

use crate::ctx::SvnClientCtx;

/// `svn_client__ensure_revprop_table`
pub fn ensure_revprop_table(
    revprop_table_in: &HashMap<String, String>,
    log_msg: &str,
    ctx: &SvnClientCtx,
) -> Result<HashMap<String, String>, SvnError> {
    let mut new_revprop_table = if !revprop_table_in.is_empty() {
        if svn_subr::properties::has_svn_prop(revprop_table_in) {
            return Err(SvnClientError::PropertyName.into());
        }
        revprop_table_in.clone()
    } else {
        HashMap::new()
    };

    new_revprop_table.insert("log".to_string(), log_msg.to_string());

    Ok(new_revprop_table)
}
