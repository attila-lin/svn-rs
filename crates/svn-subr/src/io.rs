//! `subversion/libsvn_subr/io.c`

use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::path::PathBuf;

use svn_types::NodeKind;

/// Used as an argument when creating temporary files to indicate
/// when a file should be removed.
/// `svn_io_file_del_t`
pub enum FileDel {
    /// No deletion ever
    None = 0,
    /// Remove when the file is closed
    OnClose,
    /// Remove when the associated pool is cleared
    OnPoolCleanup,
}

/// A set of directory entry data elements as returned by svn_io_get_dirents
///
/// Note that the first two fields are exactly identical to svn_io_dirent_t
/// to allow returning a svn_io_dirent2_t as a svn_io_dirent_t.
///
/// Use svn_io_dirent2_create() to create new svn_dirent2_t instances or
/// svn_io_dirent2_dup() to duplicate an existing instance.
///
/// **Note**: New fields must be added at the end to preserve binary compatibility.
/// Don't forget to update svn_io_dirent2_dup() when adding new fields
///
/// `svn_io_dirent2_t`
pub struct Dirent {
    /// The kind of this entry.
    kind: NodeKind,

    /// If @c kind is #svn_node_file, whether this entry is a special file;
    /// else FALSE.
    ///
    /// @see svn_io_check_special_path().
    special: bool,

    /// The filesize of this entry or undefined for a directory
    filesize: u64,

    /// The time the file was last modified
    mtime: i64,
}

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
///
/// `svn_io_set_file_read_only`
#[cfg(windows)]
pub fn set_file_read_only(path: &Path, ignore_enoent: bool) -> Result<(), std::io::Error> {
    use std::ffi::CString;
    use std::io;
    use windows::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_READONLY, SetFileAttributesA,
    };
    use windows::core::PCSTR;

    let s = path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path is not valid UTF-8"))?;
    let path_cstr = CString::new(s)?;
    let path_pcstr = PCSTR(path_cstr.as_ptr() as *const u8);

    unsafe {
        if SetFileAttributesA(path_pcstr, FILE_ATTRIBUTE_READONLY).is_ok() {
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

///  Writes @a nbytes bytes from @a *buf to a temporary file inside the same
/// directory as @a *final_path. Then syncs the temporary file to disk and
/// closes the file. After this rename the temporary file to @a final_path,
/// possibly replacing an existing file.
///
/// If @a copy_perms_path is not NULL, copy the permissions applied on @a
/// @a copy_perms_path on the temporary file before renaming.
///
/// If @a flush_to_disk is non-zero, do not return until the node has
/// actually been written on the disk.
///
/// @note The flush to disk operation can be very expensive on systems
/// that implement flushing on all IO layers, like Windows. Please use
/// @a flush_to_disk flag only for critical data.
///
/// `svn_io_write_atomic2`
pub fn write_atomic(
    final_path: &Path,
    buf: &[u8],
    copy_perms_path: Option<&Path>,
    flush_to_disk: bool,
) -> Result<(), std::io::Error> {
    // Create a temporary file in the same directory as final_path
    let temp_file_path = final_path.with_file_name(format!(
        "{}.tmp",
        final_path.file_name().unwrap().to_string_lossy()
    ));

    let mut temp_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_file_path)?;

    // Write the buffer to the temporary file
    temp_file.write_all(buf)?;

    // Flush the file to disk if requested
    if flush_to_disk {
        temp_file.sync_all()?;
    }

    // Close the temporary file
    temp_file.flush()?;

    // Copy permissions if requested
    if let Some(copy_perms_path) = copy_perms_path {
        let perms = fs_err::metadata(copy_perms_path)?.permissions();
        fs_err::set_permissions(&temp_file_path, perms)?;
    }

    // Rename the temporary file to the final path
    fs_err::rename(temp_file_path, final_path)?;

    Ok(())
}

/// Same as svn_io_dir_make(), but sets the hidden attribute on the
///     directory on systems that support it.
/// `svn_io_dir_make_hidden`
pub fn dir_make_hidden(path: &Path) -> Result<(), std::io::Error> {
    let metadata = fs_err::metadata(path);
    if let Err(e) = metadata {
        if e.kind() == std::io::ErrorKind::NotFound {
            fs_err::create_dir(path)?;
        } else {
            return Err(e);
        }
    }

    // Hide the directory on Windows
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        let attrs = fs_err::metadata(path)?.file_attributes();
        // fs_err::set_file_attributes(path, attrs | FILE_ATTRIBUTE_HIDDEN)?;
    }
    #[cfg(not(windows))]
    {
        todo!("Hiding directories on non-Windows systems is not implemented");
    }
    Ok(())
}
