//! `subversion/libsvn_subr/io.c`

use std::path::Path;

/// Make a file as read-only as the operating system allows.
/// @a path is the utf8-encoded path to the file. If @a ignore_enoent is
/// @c TRUE, don't fail if the target file doesn't exist.
///
/// If @a path is a symlink, do nothing.
///
/// @note If @a path is a directory, act on it as though it were a
/// file, as described above, but note that you probably don't want to
/// call this function on directories.  We have left it effective on
/// directories for compatibility reasons, but as its name implies, it
/// should be used only for files.
#[cfg(windows)]
pub fn set_file_read_only(path: &Path, ignore_enoent: bool) -> Result<(), std::io::Error> {
    use std::ffi::CString;
    use std::io;
    use windows::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_READONLY, SetFileAttributesA,
    };
    use windows::core::PCSTR;

    let path_cstr = CString::new(path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains null byte"))?;
    let path_pcstr = PCSTR(path_cstr.as_ptr() as *const u8);

    unsafe {
        if SetFileAttributesA(path_pcstr, FILE_ATTRIBUTE_READONLY).as_bool() {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(unix)]
pub fn set_file_read_only(path: &Path, ignore_enoent: bool) -> Result<(), std::io::Error> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::metadata(path);
    if let Err(e) = metadata {
        if ignore_enoent && e.kind() == std::io::ErrorKind::NotFound {
            return Ok(());
        }
        return Err(e);
    }

    let mut permissions = metadata?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(path, permissions)
}
