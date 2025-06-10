//! `authz.h`/`authz.c` bindings for SVN repositories.

use std::collections::HashMap;
use std::sync::Arc;

use bitflags::bitflags;
use bytes::Bytes;

bitflags! {
    /// Access rights in an ACL.
    ///
    /// This enum is different from and incompatible with
    /// svn_repos_authz_access_t, because it has different semantics and
    /// encodes rights that are not and should never be exposed in the
    /// public API.
    ///
    /// `authz_access_t`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AuthzAccess: u32 {
        /// Read access allows listing directory entries, reading file
        /// contents and reading properties of files and directories.
        const READ_FLAG = 0x20;

        /// Write access allows adding, removing and renaming directory
        /// entries, modifying file contents and adding, removing and
        /// modifying properties of files and directories.
        const WRITE_FLAG = 0x40;

        /// No access.
        const NONE = 0x00;

        /// Read access
        const READ = Self::READ_FLAG.bits();
        /// Write access
        const WRITE = Self::WRITE_FLAG.bits() | Self::READ_FLAG.bits();

    }
}

/// The pattern tree.  All relevant path rules are being folded into this
/// prefix tree, with a single, whole segment stored at each node.  The whole
/// tree applies to a single user only.
///
/// `node_t`
#[derive(Debug)]
pub struct Node {
    /// The segment as specified in the path rule.  During the lookup tree walk,
    /// this will compared to the respective segment of the path to check.
    segment: String,
    /// Immediate access rights granted by rules on this node and the min /
    /// max rights on any path in this sub-tree.
    rights: LimitedRights,
    /// Map of sub-segment(const char *) to respective node (node_t) for all
    /// sub-segments that have rules on themselves or their respective subtrees.
    /// NULL, if there are no rules for sub-paths relevant to the user.
    sub_nodes: Option<HashMap<String, Node>>,

    /// If not NULL, this contains the pattern-based segment sub-nodes.
    pattern_sub_nodes: Option<Box<NodePattern>>,
}

/// Since prefix arrays may have more than one hit, we need to link them
/// for fast lookup.
///
/// `sorted_pattern_t`
#[derive(Debug)]
pub struct SortedPattern {
    /// The filtered tree node carrying the prefix.
    node: Arc<Node>,
    /// Entry that is a prefix to this one or NULL
    next: Option<Box<SortedPattern>>,
}

/// Substructure of node_t.  It contains all sub-node that use patterns
/// in the next segment level. We keep it separate to save a bit of memory
/// and to be able to check for pattern presence in a single operation.
///
/// `node_pattern_t`
#[derive(Debug)]
pub struct NodePattern {
    /// If not NULL, this represents the "*" follow-segment.
    any: Option<Arc<Node>>,
    /// If not NULL, this represents the "**" follow-segment.
    any_var: Option<Arc<Node>>,
    /// If not NULL, the segments of all `sorted_pattern_t` in this array are the
    /// prefix part of "prefix*" patterns.  Sorted by segment prefix.
    prefixes: Option<Vec<SortedPattern>>,

    /// This node itself is a "**" segment and must therefore itself be added
    /// to the matching node list for the next level.
    repeat: bool,
}

/// This structure describes the access rights given to a specific user by
/// a path rule (actually the rule set specified for a path).  I.e. there is
/// one instance of this per path rule.
///
/// `path_access_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathAccess {
    /// Sequence number of the path rule that this struct was derived from.
    /// If multiple rules apply to the same path (only possible with wildcard
    /// matching), the one with the highest SEQUENCE_NUMBER wins, i.e. the latest
    /// one defined in the authz file.
    //
    /// A value of 0 denotes the default rule at the repository root denying
    /// access to everybody.  User-defined path rules start with ID 1.
    pub sequence_number: i32,

    /// Access rights of the respective user as defined by the rule set.
    pub rights: AuthzAccess,
}

/// Convenience structure combining the node-local access rights with the
/// min and max rights granted within the sub-tree.
///
/// `limited_rights_t`
#[derive(Debug)]
pub struct LimitedRights {
    /// Access granted to the current user.  If the SEQUENCE_NUMBER member is
    /// NO_SEQUENCE_NUMBER, there has been no specific path rule for this PATH
    /// but only for some sub-path(s).  There is always a rule at the root node.
    pub access: PathAccess,
    /// Minimal access rights that the user has on this or any other node in
    /// the sub-tree.  This does not take inherited rights into account.
    pub min_rights: AuthzAccess,
    /// Maximal access rights that the user has on this or any other node in
    /// the sub-tree.  This does not take inherited rights into account.
    pub max_rights: AuthzAccess,
}

impl LimitedRights {
    /// Aggregate the ACCESS spec of TARGET and RIGHTS into TARGET.  I.e. if both
    /// are specified, pick one in accordance to the precedence rules.
    ///
    /// `combine_access`
    pub fn combine_access(&mut self, other: &Self) {
        if other.access.sequence_number > self.access.sequence_number {
            self.access = other.access;
        }
    }

    /// Aggregate the min / max access rights of TARGET and RIGHTS into TARGET.
    ///
    /// `combine_right_limits`
    pub fn combine_right_limits(&mut self, rights: &Self) {
        self.max_rights |= rights.max_rights;
        self.min_rights &= rights.min_rights;
    }
}

/// Accumulated rights for (user, repository).
///
/// `authz_rights_t`
pub struct AuthzRights {
    /// The lowest level of access that the user has to every
    /// path in the repository.
    pub min_access: AuthzAccess,
    /// The highest level of access that the user has to
    /// any path in the repository
    pub max_access: AuthzAccess,
}

/// An entry in svn_authz_t's USER_RULES cache.  All members must be
/// allocated in the POOL and the latter has to be cleared / destroyed
/// before overwriting the entries' contents.
///
/// `authz_user_rules_t`
pub struct AuthzUserRules {
    /// User name for which we filtered the rules.
    /// User NULL for the anonymous user.
    pub user: Option<String>,
    /// Repository name for which we filtered the rules.
    /// May be empty but never NULL for used entries.
    pub repository: String,

    /// The combined min/max rights USER has on REPOSITORY.
    global_rights: AuthzRights,

    /// Root of the filtered path rule tree.
    /// Will remain NULL until the first usage.
    root: Option<Node>,
}

/// Reusable lookup state object. It is easy to pass to functions and
/// recycling it between lookups saves significant setup costs.
///
/// `lookup_state_t`
#[derive(Debug)]
pub struct LookupState {
    /// Rights immediately applying to this node and limits to the rights to
    /// any sub-path.
    rights: LimitedRights,

    /// Nodes applying to the path followed so far.
    current: Vec<Node>,

    /// Temporary array containing the nodes applying to the next path
    /// segment (used to build up the next contents of CURRENT).
    next: Vec<Arc<Node>>,

    /// Scratch pad for path operations.
    scratch_pad: String,

    /// After each lookup iteration, CURRENT and PARENT_RIGHTS will
    /// apply to this path.
    parent_path: String,

    /// Rights that apply at PARENT_PATH, if PARENT_PATH is not empty.
    parent_rights: LimitedRights,
}

impl LookupState {
    // /// `create_lookup_state`
    // fn new() -> Self {
    //     Self::default()
    // }

    /// If PREFIX is indeed a prefix (or exact match) or SEGMENT, add the
    /// node in PREFIX to STATE.
    pub fn add_if_prefix_matches(&mut self, prefix: &SortedPattern, segment: &str) {
        let node = &prefix.node;
        if node.segment.len() <= segment.len() && node.segment.starts_with(segment) {
            self.add_next_node(Some(node.clone()));
        }
    }

    /// Scan the PREFIXES array of node_t* for all entries whose SEGMENT members
    //  are prefixes of SEGMENT.  Add these to STATE for the next tree level.
    /// `add_prefix_matches`
    pub fn add_prefix_matches(&mut self, prefixes: &[SortedPattern], segment: &str) {
        for prefix in prefixes {
            self.add_if_prefix_matches(prefix, segment);
        }
    }

    /// Add NODE to the list of NEXT nodes in STATE.
    /// NODE may be NULL in which case this is a no-op.
    /// Also update and aggregate the access rights data
    /// for the next path segment.
    ///
    /// `add_next_node`
    pub fn add_next_node(&mut self, node: Option<Arc<Node>>) {
        if let Some(node) = node {
            // The rule with the highest sequence number is the one that applies.
            // Not all nodes that we are following have rules that apply directly
            // to this path but are mere intermediates that may only have some
            // matching deep sub-node.
            self.rights.combine_access(&node.rights);

            // The rule tree node can be seen as an overlay of all the nodes that
            // we are following.  Any of them _may_ match eventually, so the min/
            // max possible access rights are a combination of all these sub-trees.
            self.rights.combine_right_limits(&node.rights);

            // NODE is now enlisted as a (potential) match for the next segment.
            self.next.push(node.clone());

            // Variable length sub-segment sequences apply to the same node as
            // they match empty sequences as well.
            if let Some(pattern_sub_nodes) = &node.pattern_sub_nodes {
                if let Some(any_var) = &pattern_sub_nodes.any_var {
                    let node = any_var;

                    // This is non-recursive due to ACL normalization.
                    self.rights.combine_access(&node.rights);
                    self.rights.combine_right_limits(&node.rights);
                    self.next.push(node.clone());
                }
            }
        }
    }
}

/// The segment type.
pub enum AuthzRuleSegmentKind {
    /// A literal string match.
    /// The path segment must exactly match the pattern.
    ///
    /// Note: Make sure this is always the first constant in the
    /// enumeration, otherwise rules that match the repository
    /// root will not sort first in the ACL list and the implicit
    /// default no-access ACE will not be applied correctly.
    Literal,
    /// A prefix match: a literal string followed by '*'.
    //  The path segment must begin with the literal prefix.
    Prefix,

    /// A suffix match: '*' followed by a literal string.
    //  The path segment must end with the literal suffix.
    //  The pattern is stored reversed, so that the matching code can
    //  perform a prefix match on the reversed path segment.
    Suffix,
    /// '*'
    //  Matches any single non-empty path segment.
    //  The pattern will be an empty string.
    AnySegment,

    /// '**'
    /// Matches any sequence of zero or more path segments.
    /// The pattern will be an empty string.
    AnyRecursive,
    /// Any other glob/fnmatch pattern.
    Fnmatch,
}

/// Rule path segment descriptor.
/// `authz_rule_segment_t`
pub struct AuthzRuleSegment {
    pub kind: AuthzRuleSegmentKind,
    pub pattern: String,
}

/// Rule path descriptor.
///
/// `authz_rule_t`
pub struct AuthzRule {
    /// The repository that this rule applies to. This will be the empty
    /// string if the rule did not name a repository. The
    /// repository name is interned.
    pub repos: String,

    /// The number of segments in the rule path.
    pub len: i32,

    /// The array of path segments for this rule. Will be NULL for the
    /// repository root.
    pub path: AuthzRuleSegment,
}

/// An access control list defined by access rules.
///
/// `authz_acl_t`
pub struct AuthzAcl {
    /// The sequence number of the ACL stores the order in which access
    /// rules were defined in the authz file. The authz lookup code
    /// selects the highest-numbered ACL from amongst a set of equivalent
    /// matches.
    sequence_number: i32,

    /// The parsed rule.
    rule: AuthzRule,
    /// Access rights for anonymous users.
    bas_anonymous_access: bool,
    anonymous_access: AuthzAccess,

    /// Access rights for authenticated users.
    has_neg_access: bool,
    neg_access: AuthzAccess,

    /// All other user- or group-specific access rights.
    /// Aliases are replaced with their definitions, rules for the same
    /// user or group are merged.
    user_access: Vec<AuthzAce>,
}

/// An access control entry in authz_acl_t::user_access.
///
/// `authz_ace_t`
pub struct AuthzAce {
    /// The name of the alias, user or group that this ACE applies to.
    pub name: String,
    /// The set of group members, when NAME is the name of a group.
    /// We store this reference in the ACE to save a hash lookup when
    /// resolving access for group ACEs.
    pub members: HashMap<String, Option<u32>>,

    /// True if this is an inverse-match rule.
    inverted: bool,
    /// The access rights defined by this ACE.
    access: AuthzAccess,
}

/// Immutable authorization info
///
/// `authz_full_t`
pub struct AuthzFull {
    /// All ACLs from the authz file, in the order of definition.
    acls: Vec<AuthzAcl>,
}

/// Dynamic authorization info
///
/// `svn_authz_t`
pub struct SvnAuthz {
    /// The parsed and pre-processed contents of the authz file.
    full: AuthzFull,

    ///  Identifies the authz model content
    /// (a hash value that can be used for e.g. cache lookups).
    authz_id: Bytes,

    /// Rules filtered for a particular user-repository combination.
    /// May be NULL.
    filtered: Option<Arc<AuthzUserRules>>,
}
