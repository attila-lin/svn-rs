use svn_subr::Version;

/// The RA layer
pub trait RaLayer {
    /// This field should always remain first in the vtable.
    fn get_version() -> Version
    where
        Self: Sized;
}
