use std::path::Path;
use std::path::PathBuf;

use svn_types::error::SvnClientError;
use svn_types::error::SvnError;
use svn_types::error::SvnMiscError;
use svn_types::error::SvnNodeError;
use svn_wc::status::Status;
use svn_wc::status::StatusKind;

/// `can_delete_baton_t`
pub struct CanDeleteBaton {
    root_abspath: PathBuf,
    target_missing: bool,
}

impl CanDeleteBaton {
    pub fn find_undletables(
        cdt: &mut Self,
        local_abspath: &Path,
        status: &Status,
    ) -> Result<(), SvnError> {
        if status.node_status == StatusKind::Missing {
            if cdt.root_abspath == local_abspath {
                cdt.target_missing = true;
            }
        }

        // check for error0ful states.
        if status.node_status == StatusKind::Obstructed {
            return Err(SvnNodeError::UnexpectedKind.into());
        } else if !status.versioned {
            return Err(SvnMiscError::UnversionedResource.into());
        } else if (status.node_status == StatusKind::Added
            || status.node_status == StatusKind::Replaced)
            || status.text_status == StatusKind::Normal
                && (status.prop_status == StatusKind::Normal
                    || status.prop_status == StatusKind::None)
        {
            // Unmodified copy. Go ahead, remove it
        } else if status.node_status != StatusKind::Normal
            && status.node_status != StatusKind::Deleted
            && status.node_status != StatusKind::Missing
        {
            return Err(SvnClientError::Modified.into());
        }

        Ok(())
    }
}
