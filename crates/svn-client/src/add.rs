//! wrappers around wc add/mkdir functionality.

use std::{collections::HashMap, path::Path};

use crate::{Error, ctx::SvnClientCtx};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDel {
    None = 0,
    OnClose,
    PoolCleanup,
}

/// Represents the kind and special status of a directory entry.
///
/// Note that the first two fields are exactly identical to svn_io_dirent2_t
/// to allow returning a svn_io_dirent2_t as a svn_io_dirent_t.
///
/// @since New in 1.3.
///
///    `svn_io_dirent_t`
pub struct Dirent {
    /// the kind of this entry.
    kind: NodeKind,
    special: bool,
}

/// Split PROPERTY and store each individual value in PROPS.
/// Allocates from POOL.
pub fn split_props(property: &str) -> Vec<String> {
    todo!()
}

/// PROPVALS is a hash mapping char * property names to const char * property
/// values.  PROPERTIES can be empty but not NULL.
///
/// If FILENAME doesn't match the filename pattern PATTERN case insensitively,
/// the do nothing.  Otherwise for each 'name':'value' pair in PROPVALS, add
/// a new entry mappying 'name' to a svn_string_t * wrapping the 'value' in
/// PROPERTIES.  The svn_string_t is allocated in the pool used to allocate
/// PROPERTIES, but the char *'s from PROPVALS are re-used in PROPERTIES.
/// If PROPVALS contains a 'svn:mime-type' mapping, then set *MIMETYPE to
/// the mapped value.  Likewise if PROPVALS contains a mapping for
/// svn:executable, then set *HAVE_EXECUTABLE to TRUE.
///
/// Use SCRATCH_POOL for temporary allocations.
fn get_auto_props_for_pattern(
    filename: &str,
    pattern: &str,
    properties: HashMap<String, String>,
    mimetype: &mut Option<String>,
    have_executable: &mut bool,
) -> () {
    let pattern = glob::Pattern::new(pattern).unwrap();
    if !pattern.matches(filename) {
        return;
    }

    for (propname, propval) in properties {
        let propval_str = value.clone();

        properites.insert(propname, propval_str);
        if propname == "mime-type" {
            mimetype = Some(propval);
        } else if propname == "executable" {
            have_executable = true;
        }
    }
}

pub fn get_paths_auto_props(
    properties: HashMap<String, String>,
    mimetype: Option<String>,
    path: &Path,
) -> () {
    let mut have_executable = false;
    let mut mimetype = None;

    if let Some(autoprops) = autoprops {
        for (pattern, propvals) in autopros {
            get_auto_props_for_pattern(
                path.base(),
                pattern,
                properties,
                &mut mimetype,
                &mut have_executable,
            );
        }
    }

    // if mimetype has not bben set check the file
    if mimetype.is_none() {
        mimetype = svn_subr::io::detect_mimetype(path, &HashMap::new())?;
        // If we got no mime-type, or if it is "applicateion/octet-steram",
        // try to get the mime-type from libmagic.
        // if
    }

    if let Some(mimetype) = mimetype {
        properties.insert("svn:mime-type".to_string(), mimetype.to_string());
    }
}

impl SvnClientCtx {
    pub fn add_file(
        &self,
        local_abspath: &Path,
        autoprops: &HashMap<String, Props>,
        no_autoprops: bool,
    ) -> Result<(), Error> {
        /* Check to see if this is a special file. */
        let (kind, is_special) = svn_subr::io::check_special_path(local_abspath)?;

        /* Determine the properties that the file should have */
        if is_special {
            mimetype = None;
            properties = HashMap::new();
            properties.insert("special".into(), "*".into());
        } else {
            file_autoprops = None;

            if !no_autoprops {
                if autoprops.is_none() {
                    get_all_auto_props();
                }
            } else {
                file_autoprops = autoprops;
            }
            /* This may fail on write-only files:
            we open them to estimate file type. */
            get_paths_auto_props(properties, mimetype, path);
        }
        // Add the file
        self.wc_ctx
            .add_from_disk(local_abspath, properties, false, self.notify_baton)?;

        Ok(())
    }
}
