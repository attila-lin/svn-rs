use svn_subr::error::ClientError;
use svn_subr::error::MiscError;
use svn_subr::error::NodeError;
use svn_wc::status::Status;
use svn_wc::status::StatusKind;

/// `can_delete_baton_t`
pub struct CanDeleteBaton {
    root_abspath: PathBuf,
    target_missing: bool,
}

impl CanDeleteBaton {
    pub fn find_undletables(cdt: Self, local_abspath: &Path, status: WcStatus) {
        if status.node_status == StatusKind::Missing {
            if cdt.root_abspath == local_abspath {
                cdt.target_missing = true;
            }
        }

        // check for error0ful states.
        if status.node_status == StatusKind::Obstructed {
            return Err(NodeError::UnexpectedKind);
        } else if !status.versioned {
            return Err(MiscError::UnversionedResource);
        } else if (status.node_status == StatusKind::Added || node_status == StatusKind::Replaced)
            || status.text_status == StatusKind::Normal
                && (status.prop_status == StatusKind::Normal
                    || status.prop_status == StatusKind::None)
        {
            // Unmodified copy. Go ahead, remove it
        } else if status.node_status != StatusKind::Normal
            && status.node_status != StatusKind::Deleted
            && status.node_status != StatusKind::Missing
        {
            return Err(ClientError::Modified);
        }

        Ok(())
    }
}
