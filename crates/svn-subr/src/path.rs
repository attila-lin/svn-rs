//! `path.c`

pub fn is_repos_relative_url(path: &str) -> bool {
    path.starts_with("^/")
}
