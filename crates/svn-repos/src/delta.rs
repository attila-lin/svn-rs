//! an editor driver for expressing differences between two trees
//!
//! `delta.c`

use svn_fs::FsRoot;

/// The standard size of one svndiff window.
const WINDOW_SIZE: usize = 1024 * 100;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum DeltaError {}

// /** Use the provided @a editor and @a edit_baton to describe the changes
//  * necessary for making a given node (and its descendants, if it is a
//  * directory) under @a src_root look exactly like @a tgt_path under
//  * @a tgt_root.  @a src_entry is the node to update.  If @a src_entry
//  * is empty, then compute the difference between the entire tree
//  * anchored at @a src_parent_dir under @a src_root and @a tgt_path
//  * under @a tgt_root.  Else, describe the changes needed to update
//  * only that entry in @a src_parent_dir.  Typically, callers of this
//  * function will use a @a tgt_path that is the concatenation of @a
//  * src_parent_dir and @a src_entry.
//  *
//  * @a src_root and @a tgt_root can both be either revision or transaction
//  * roots.  If @a tgt_root is a revision, @a editor's set_target_revision()
//  * will be called with the @a tgt_root's revision number, else it will
//  * not be called at all.
//  *
//  * If @a authz_read_func is non-NULL, invoke it before any call to
//  *
//  *    @a editor->open_root
//  *    @a editor->add_directory
//  *    @a editor->open_directory
//  *    @a editor->add_file
//  *    @a editor->open_file
//  *
//  * passing @a tgt_root, the same path that would be passed to the
//  * editor function in question, and @a authz_read_baton.  If the
//  * @a *allowed parameter comes back TRUE, then proceed with the planned
//  * editor call; else if FALSE, then invoke @a editor->absent_file or
//  * @a editor->absent_directory as appropriate, except if the planned
//  * editor call was open_root, throw SVN_ERR_AUTHZ_ROOT_UNREADABLE.
//  *
//  * If @a text_deltas is @c FALSE, send a single @c NULL txdelta window to
//  * the window handler returned by @a editor->apply_textdelta().
//  *
//  * If @a depth is #svn_depth_empty, invoke @a editor calls only on
//  * @a src_entry (or @a src_parent_dir, if @a src_entry is empty).
//  * If @a depth is #svn_depth_files, also invoke the editor on file
//  * children, if any; if #svn_depth_immediates, invoke it on
//  * immediate subdirectories as well as files; if #svn_depth_infinity,
//  * recurse fully.
//  *
//  * If @a entry_props is @c TRUE, accompany each opened/added entry with
//  * propchange editor calls that relay special "entry props" (this
//  * is typically used only for working copy updates).
//  *
//  * @a ignore_ancestry instructs the function to ignore node ancestry
//  * when determining how to transmit differences.
//  *
//  * Before completing successfully, this function calls @a editor's
//  * close_edit(), so the caller should expect its @a edit_baton to be
//  * invalid after its use with this function.
//  *
//  * Do any allocation necessary for the delta computation in @a pool.
//  * This function's maximum memory consumption is at most roughly
//  * proportional to the greatest depth of the tree under @a tgt_root, not
//  * the total size of the delta.
//  *
//  * ### svn_repos_dir_delta2 is mostly superseded by the reporter
//  * ### functionality (svn_repos_begin_report3 and friends).
//  * ### svn_repos_dir_delta2 does allow the roots to be transaction
//  * ### roots rather than just revision roots, and it has the
//  * ### entry_props flag.  Almost all of Subversion's own code uses the
//  * ### reporter instead; there are some stray references to the
//  * ### svn_repos_dir_delta[2] in comments which should probably
//  * ### actually refer to the reporter.
//  *
//  * @since New in 1.5.
//
// `svn_repos_dir_delta2`
pub fn dir_delta(
    src_root: &FsRoot,
    src_parent_dir: &str,
    src_entry: &str,
    tgt_root: &FsRoot,
    tgt_fullpath: &str,
) -> Result<(), DeltaError> {
    Ok(())
}
