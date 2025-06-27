//! `layout.c`

use std::path::PathBuf;

use svn_ra::Reporter;
use svn_types::{Depth, RevisionNumber};
use svn_wc::WcContext;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum LayoutError {}

/// `layout_item_t`
pub struct LayoutItem {
    local_abspath: PathBuf,
    url: String,
    revision: Option<RevisionNumber>,
    depth: Depth,
}

/// `client_layout_baton_t`
pub struct LayoutBaton {
    root_abspath: PathBuf,
    wc_ctx: WcContext,
    repos_root_url: String,

    stack: Vec<LayoutItem>,
}

/// `svn_client__layout_func_t`
type LayoutFunc = Box<dyn FnMut(&mut LayoutBaton) -> Result<(), LayoutError>>;

impl Reporter for LayoutBaton {
    // `layout_set_path`
    fn set_path(
        &mut self,
        path: &str,
        revision: Option<i64>,
        depth: Depth,
        start_empty: bool,
        lock_token: Option<&str>,
    ) -> Result<(), LayoutError> {
        let _ = lock_token;
        let local_abspath = self.root_abspath.join(path);
        let url = if !self.stack.is_empty() {
            let last = self.stack.last().unwrap();
            format!("{}/{}", last.url, path)
        } else {
            format!("{}/{}", self.repos_root_url, path)
        };
        let it = LayoutItem {
            local_abspath,
            depth,
            revision,
            url,
        };

        self.stack.push(it);

        todo!()
    }

    fn delete_path(&mut self, path: &str) -> Result<(), svn_types::SvnError> {
        todo!()
    }

    fn link_path(
        &mut self,
        path: &str,
        url: &str,
        revision: Option<i64>,
        depth: Depth,
        start_empty: bool,
        lock_token: Option<&str>,
    ) -> Result<(), svn_types::SvnError> {
        todo!()
    }

    fn finish_report(&mut self) -> Result<(), svn_types::SvnError> {
        todo!()
    }

    fn abort_report(&mut self) -> Result<(), svn_types::SvnError> {
        todo!()
    }
}
