//! `libsvn_client/cmdline.c`

use svn_subr::opt;
use svn_subr::path;

use crate::ctx::SvnClientCtx;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("'{0}' is just a peg revision. Maybe try '{0}@' instead?")]
    BadFilename(String),
}

/// `svn_client_args_to_target_array2`
pub fn args_to_target(
    args: (),
    known_targets: &[&str],
    ctx: &SvnClientCtx,
    keep_last_origpath_on_truepath_collision: bool,
) -> Result<Vec<String>, Error> {
    let mut rel_url_found = false;

    for target in known_targets {
        if is_repos_relative_url(target) {
            rel_url_found = true;
            break;
        }
    }

    let mut output_targets = Vec::new();
    for target in known_targets {
        // Relative urls will be canonicalized when they are resolved later in
        // the function
        if path::is_repos_relative_url(target) {
            output_targets.push(target.to_owned());
        } else {
            // This is needed so that the target can be properly canonicalized,
            // otherwise the canonicalization does not treat a ".@BASE" as a "."
            // with a BASE peg revision, and it is not canonicalized to "@BASE".
            // If any peg revision exists, it is appended to the final
            // canonicalized path or URL.  Do not use svn_opt_parse_path()
            // because the resulting peg revision is a structure that would have
            // to be converted back into a string.  Converting from a string date
            // to the apr_time_t field in the svn_opt_revision_value_t and back to
            // a string would not necessarily preserve the exact bytes of the
            // input date, so its easier just to keep it in string form.
            let (true_target, peg_rev) = opt::split_arg_at_peg_revision(target);

            /* Reject the form "@abc", a peg specifier with no path. */
            if target.is_empty() && !peg_rev.is_empty() {
                return Err(Error::BadFilename(target.to_string()));
            }

            /* URLs and wc-paths get treated differently. */
            if path::is_url(true_target) {
                true_target = opt::arg_canonicalize_url(true_target)
                    .map_err(|_| Error::BadFilename(true_target.to_string()))?;
            } else {
                todo!()
            }
        }
    }

    Ok(output_targets)
}
