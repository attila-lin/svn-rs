use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use svn_ra::svn::session::RaSession;
use svn_types::NodeKind;
use svn_types::error::SvnClientError;
use svn_types::error::SvnError;
use svn_types::error::SvnMiscError;
use svn_types::error::SvnNodeError;
use svn_types::error::SvnWcError;
use svn_wc::status::Status;
use svn_wc::status::StatusKind;
use url::Url;

use crate::commit::CommitItem;
use crate::ctx::SvnClientCtx;

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

/// Check whether LOCAL_ABSPATH is an external and raise an error if it is.
///
///    A file external should not be deleted since the file external is
///    implemented as a switched file and it would delete the file the
///    file external is switched to, which is not the behavior the user
///    would probably want.
///
///    A directory external should not be deleted since it is the root
///    of a different working copy.
///
///
pub fn check_external(local_abspath: &Path, ctx: &SvnClientCtx) -> Result<(), SvnError> {
    let external_kind = ctx.wc_ctx.read_external_info(local_abspath)?;

    if external_kind != NodeKind::None {
        return Err(SvnWcError::CannotDeleteFileExternal.into());
    }
    Ok(())
}

/// Verify that the path can be deleted without losing stuff,
///    i.e. ensure that there are no modified or unversioned resources
///    under PATH.  This is similar to checking the output of the status
///    command.  CTX is used for the client's config options.  POOL is
///    used for all temporary allocations.
pub fn can_delete_node(local_abspath: &Path, ctx: SvnClientCtx) -> Result<bool, SvnError> {
    /* Use an infinite-depth status check to see if there's anything in
    or under PATH which would make it unsafe for deletion.  The
    status callback function find_undeletables() makes the
    determination, returning an error if it finds anything that shouldn't
    be deleted. */
    let mut ignores: Vec<_> = svn_wc::get_default_ignores(&ctx.config)?;

    let cdt = CanDeleteBaton {
        root_abspath: local_abspath.to_path_buf(),
        target_missing: false,
    };

    svn_wc::walk_status();

    Ok(cdt.target_missing)
}

pub fn path_dreive_cb_func() {
    todo!()
}

pub fn single_repos_delete(
    ra_session: &impl RaSession,
    base_uri: &Url,
    relpaths: &[String],
    revprop_table: &HashMap<String, ()>,
    commit_callback: SvnCommitCallback,
    ctx: &SvnClientCtx,
) -> Result<(), SvnError> {
    /* Create new commit items and add them to the array. */
    let log_msg = if ctx.is_has_log_func() {
        let commit_items = Vec::with_capacity(relpaths.len());

        for relpath in relpaths {
            let item = CommitItem {
                url: base_uri.join(relpath)?,
                state_flags: SvnCommitItemStateFlags::Deleted,
                ..Default::default()
            };
            commit_items.push(item);
        }
        get_log_msg(tmp_file, commit_items, ctx)
    } else {
        ""
    };

    util::ensure_revprop_table(commit_revprops, revprop_table, log_msg)?;

    // Fetch RA commit editor
    svn_ra::register_editor_shim_callbacks(ra_session)?;

    svn_ra::get_commit_editor()?;

    // Call the path-base editor driver
    svn_delta::path_deriver(
        editor,
        edit_baton,
        relpaths,
        true,
        path_driver_cb_func,
        None,
    );
    todo!()
}
