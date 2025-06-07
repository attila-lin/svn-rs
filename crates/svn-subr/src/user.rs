//! port `svn_user.h`/`user.c`

pub fn get_name() -> String {
    whoami::username()
}

pub fn get_homedir() -> Option<String> {
    let username = get_name();
    homedir::home(&username)
        .expect("Failed to get home directory")
        .map(|path| path.to_string_lossy().to_string())
}
