//! util.c
//!

use std::collections::HashMap;
use std::path::Path;

use svn_types::RevisionNumber;
use svn_types::SvnError;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Pathrev {
    repos_root_url: Url,
    repos_uuid: Uuid,
    rev: RevisionNumber,
    url: Url,
}
impl Pathrev {
    pub fn new(repos_root_url: Url, repos_uuid: Uuid, rev: RevisionNumber, url: Url) -> Self {
        Pathrev {
            repos_root_url,
            repos_uuid,
            rev,
            url,
        }
    }
    pub fn create_with_session(
        ra_session: &SvnRaSession,
        rev: RevisionNumber,
        url: Url,
    ) -> Result<Self, SvnError> {
        let repos_root_url = ra_session.get_repos_root_url()?;
        let repos_uuid = ra_session.get_repos_uuid()?;
        Ok(Pathrev::new(repos_root_url, repos_uuid, rev, url))
    }

    pub fn join_relpath(&self, relpath: &str) -> Result<Self, SvnError> {
        Self::new(
            self.repos_root_url,
            self.repos_uuid,
            self.rev,
            svn_subr::path::add_component(&self.url, relpath)?,
        )
    }
}
#[derive(Debug, Clone)]
pub struct MergeSource {
    loc1: Pathrev,
    loc2: Pathrev,
    ancestral: bool,
}

pub fn wc_node_get_base(wc_ctx: &WcCtx, wc_abspath: &Path) -> Result<Pathrev, SvnError> {
    let base_p = wc_ctx.get_base(None, wc_abspath);
    if base_p.repos_root_url.is_none() {
        return Err(SvnError::new("Base path not found"));
    }

    Ok(base_p)
}

/// `svn_client__assert_homogeneous_target_type`
pub fn assert_homogeneous_target_type(targets: &[&str]) -> Result<(), SvnError> {
    let mut wc_parent = false;
    let mut url_parent = false;

    for target in targets {
        if !svn_subr::path::is_url(target) {
            wc_parent = true;
        } else {
            url_parent = true;
        }
        if wc_parent && url_parent {
            return Err(SvnError::new(
                "Cannot mix repository and working copy targets",
            ));
        }
    }

    Ok(())
}

///shim_callbacks_baton
pub struct ShimCallbacksBaton {
    pub wc_ctx: WcCtx,
    pub relpath_map: HashMap<String, String>,
}

impl ShimCallbacksBaton {
    fn fetch_props_func(
        self,
        path: &Path,
        base_revision: RevisionNumber,
    ) -> Result<HashMap<String, String>, SvnError> {
        todo!("Implement fetch_props_func");
    }
}
