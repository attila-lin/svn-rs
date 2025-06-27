/// Split PROPERTY and store each individual value in PROPS.
/// Allocates from POOL.
pub fn split_props(property: &str) -> Vec<String> {
    todo!()
}

/* PROPVALS is a hash mapping char * property names to const char * property
   values.  PROPERTIES can be empty but not NULL.

   If FILENAME doesn't match the filename pattern PATTERN case insensitively,
   the do nothing.  Otherwise for each 'name':'value' pair in PROPVALS, add
   a new entry mappying 'name' to a svn_string_t * wrapping the 'value' in
   PROPERTIES.  The svn_string_t is allocated in the pool used to allocate
   PROPERTIES, but the char *'s from PROPVALS are re-used in PROPERTIES.
   If PROPVALS contains a 'svn:mime-type' mapping, then set *MIMETYPE to
   the mapped value.  Likewise if PROPVALS contains a mapping for
   svn:executable, then set *HAVE_EXECUTABLE to TRUE.

   Use SCRATCH_POOL for temporary allocations.
*/
fn get_auto_props_for_pattern(
    filename: &str,
    pattern: &str,
    properties: HashMap<String, String>,
    // mimetype:
) -> () {
    let pattern = glob::Pattern::new(pattern).unwrap();
    if !pattern.matches(filename) {
        return;
    }

    for (name, value) in properties {
        todo!()
    }

    todo!()
}
