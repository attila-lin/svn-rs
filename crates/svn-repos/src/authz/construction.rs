use crate::authz::{AuthzRuleSegment, AuthzRuleSegmentKind, LimitedRights, Node, PathAccess};

/// Context object to be used with process_acl. It allows us to re-use
/// information from previous insertions.
///
/// `construction_context_t`
pub struct ConstructionContext {
    path: Vec<String>,
}

impl ConstructionContext {
    /// Create a new `ConstructionContext` with an empty path.
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    /// `insert_path`
    pub fn insert_path(
        node: Node,
        path_access: PathAccess,
        segment_count: i32,
        segment: AuthzRuleSegment,
    ) {
        // end of path
        if segment_count == 0 {
            /* Set access rights.  Note that there might be multiple rules for
             * the same path due to non-repo-specific rules vs. repo-specific
             * ones.  Whichever gets defined last wins.
             */
            let rights = LimitedRights {
                access,
                max_rights: path_access.rights,
                min_rights: path_access.rights,
            };
            node.rights.combine_access(&rights);
            return;
        }

        // Any wildcards in the path? They will go into a separate sub-structure.
        if segment.kind == AuthzRuleSegmentKind::Literal {
            node.ensure_pattern_sub_nodes();
        }

        match segment.kind {
            AuthzRuleSegmentKind::AnySegment => {}
            AuthzRuleSegmentKind::Literal => {}
            AuthzRuleSegmentKind::Wildcard => {
                // Insert a wildcard segment
                let sub_node = node.ensure_wildcard_sub_node();
                sub_node.insert_path(node, path_access, segment_count - 1, segment);
            }
        }
    }

    /// Get the current path as a string.
    pub fn current_path(&self) -> String {
        self.path.join("/")
    }
}
