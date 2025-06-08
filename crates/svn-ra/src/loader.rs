//! port `subversion/libsvn_ra/ra_loader.c`

use crate::ra_layer::RaLayer;
pub struct RaLibrary {
    /// the name of this RA library (e.g. "neon" or "local")
    ra_name: String,
    /// schemes
    schemes: Vec<String>,
    /// the initialization function if linked in; otherwise, NULL
    init_func: Option<Box<dyn RaLayer>>,
    compat_init_func: Option<fn() -> i32>,
}
