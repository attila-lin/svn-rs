/** Options to control the behaviour of the file diff routines.
 *
 * @since New in 1.4.
 *
 * @note This structure may be extended in the future, so to preserve binary
 * compatibility, users must not allocate structs of this type themselves.
 * @see svn_diff_file_options_create().
 *
 * @note Although its name suggests otherwise, this structure is used to
 *       pass options to file as well as in-memory diff functions.
 */
/// `svn_diff_file_options_t`
pub struct DiffFileOptions {
    ignore_space: bool,
    ignore_eol_style: bool,
    show_c_function: bool,
    context_size: i32,
}
