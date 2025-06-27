mod crawler;
mod error;
pub mod files;
mod ops;
pub use error::AdmError;

/*** Names and file/dir operations in the administrative area. ***/

/** The files within the administrative subdir. **/
const ADM_FORMAT: &str = "format";
const ADM_ENTRIES: &str = "entries";
const ADM_TMP: &str = "tmp";
const ADM_PRISTINE: &str = "pristine";
const ADM_NONEXISTENT_PATH: &str = "nonexistent-path";
const ADM_EXPERIMENTAL: &str = "experimental";

/// The basename of the ".prej" file, if a directory ever has property
/// conflicts.  This .prej file will appear *within* the conflicted
/// directory.
const THIS_DIR_PREJ: &str = "dir_conflicts";

/// The default name of the WC admin directory. This name is always
/// checked by svn_wc_is_adm_dir.
const DEFAULT_ADM_DIR_NAME: &str = ".svn";

/// The name that is actually used for the WC admin directory.  The
/// commonest case where this won't be the default is in Windows
/// ASP.NET development environments, which used to choke on ".svn".
const ADM_DIR_NAME: &str = DEFAULT_ADM_DIR_NAME;

pub struct Adm;

impl Adm {}
