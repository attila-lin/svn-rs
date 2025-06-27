use url::Url;

/// Return TRUE if @a parent_uri is an ancestor of @a child_uri or
/// the uris are equal, and FALSE otherwise.
///
/// `svn_uri__is_ancestor`
pub fn is_ancestor(parent_uri: &Url, child_uri: &Url) -> bool {
    // Both URLs must have the same scheme, host, and port to be considered related
    if parent_uri.scheme() != child_uri.scheme()
        || parent_uri.host() != child_uri.host()
        || parent_uri.port() != child_uri.port()
    {
        return false;
    }

    let parent_path = parent_uri.path();
    let child_path = child_uri.path();

    // If paths are exactly equal, parent is an ancestor (same URI)
    if parent_path == child_path {
        return true;
    }

    // Ensure parent path ends with '/' for proper prefix matching
    let parent_path_normalized = if parent_path.ends_with('/') {
        parent_path
    } else {
        // Create a temporary string with trailing slash
        let mut temp = String::from(parent_path);
        temp.push('/');
        return child_path.starts_with(&temp);
    };

    // Check if child path starts with parent path
    child_path.starts_with(parent_path_normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ancestor() {
        // Test cases based on Subversion's uri_ancestor_tests
        let test_cases = vec![
            // (parent, child, expected)
            ("http://test", "http://test", true),     // Same URL
            ("http://test", "http://taste", false),   // Different paths
            ("http://test", "http://test/foo", true), // Parent is ancestor
            ("http://test", "file://test/foo", false), // Different schemes
            ("http://test", "http://testf", false),   // Not a proper ancestor
            ("http://example.com", "http://test", false), // Different hosts
            ("http://server", "http://server/q", true), // Simple ancestor
            ("svn://server", "http://server/q", false), // Different schemes
            ("http://foo/bar", "http://foo", false),  // Child is not descendant
            ("http://foo/bar", "http://foo/ba", false), // Partial match but not ancestor
        ];

        for (parent_str, child_str, expected) in test_cases {
            let parent = Url::parse(parent_str).unwrap();
            let child = Url::parse(child_str).unwrap();
            let result = is_ancestor(&parent, &child);

            assert_eq!(
                result, expected,
                "is_ancestor({}, {}) expected {}, got {}",
                parent_str, child_str, expected, result
            );
        }
    }

    #[test]
    fn test_is_ancestor_with_trailing_slash() {
        // Test with and without trailing slashes
        let parent_with_slash = Url::parse("http://test/").unwrap();
        let parent_without_slash = Url::parse("http://test").unwrap();
        let child = Url::parse("http://test/foo").unwrap();

        assert!(is_ancestor(&parent_with_slash, &child));
        assert!(is_ancestor(&parent_without_slash, &child));
    }

    #[test]
    fn test_is_ancestor_same_url() {
        let url = Url::parse("http://example.com/path").unwrap();
        assert!(is_ancestor(&url, &url));
    }

    #[test]
    fn test_is_ancestor_different_ports() {
        let parent = Url::parse("http://server:8080/path").unwrap();
        let child = Url::parse("http://server:9090/path/sub").unwrap();
        assert!(!is_ancestor(&parent, &child));
    }
}
