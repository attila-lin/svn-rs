//! `path.c`

pub fn is_repos_relative_url(path: &str) -> bool {
    path.starts_with("^/")
}

pub fn is_url(path: &str) -> bool {
    url::Url::parse(path).is_ok()
}
