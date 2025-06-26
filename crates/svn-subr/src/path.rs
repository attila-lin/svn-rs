//! `path.c`
use url::Url;

pub fn is_repos_relative_url(path: &str) -> bool {
    path.starts_with("^/")
}

pub fn is_url(path: &str) -> bool {
    Url::parse(path).is_ok()
}

/// Extend @a url by @a component, URI-encoding that @a component
///  * before adding it to the @a url; return the new @a url, allocated in
///  * @a pool.  If @a component is @c NULL, just return a copy of @a url,
///  * allocated in @a pool.
///  *
///  * @a component need not be a single path segment, but if it contains
///  * multiple segments, they must be separated by '/'.  @a component
///  * should not begin with '/', however; if it does, the behavior is
///  * undefined.
///  *
///  * @a url must be in canonical format; it may not have a trailing '/'.
///  *
///  * @note To add a component that is already URI-encoded, use
///  *       <tt>svn_path_join(url, component, pool)</tt> instead.
///  *
///  * @note gstein suggests this for when @a component begins with '/':
///  *
///  *       "replace the path entirely
///  *        https://example.com:4444/base/path joined with /leading/slash,
///  *        should return: https://example.com:4444/leading/slash
///  *        per the RFCs on combining URIs"
///  *
///  *       We may implement that someday, which is why leading '/' is
///  *       merely undefined right now.
///  *
///  * @since New in 1.6.
pub fn url_add_component(url: Url, component: &str) -> Url {
    url.join(component).unwrap()
}

/// `svn_path_url_add_component2`
pub fn add_component() {
    todo!()
}
