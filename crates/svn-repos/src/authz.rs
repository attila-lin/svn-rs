//! `authz.h`/`authz.c` bindings for SVN repositories.

pub mod construction;
pub mod info;
pub mod parse;
pub use parse::AuthzParser;

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
#[derive(Debug, Default)]
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
    sub_nodes: Option<HashMap<String, Box<Node>>>,

    /// If not NULL, this contains the pattern-based segment sub-nodes.
    pattern_sub_nodes: Option<Box<NodePattern>>,
}

impl Node {
    /// Create a new tree node for SEGMENT.
    /// Note: SEGMENT->pattern is always interned and therefore does not
    /// have to be copied into the result pool.
    ///
    /// `create_node`
    fn create_node(segment: &AuthzRuleSegment) -> Self {
        Self {
            segment: segment.pattern.clone(),
            rights: LimitedRights::default(),
            sub_nodes: None,
            pattern_sub_nodes: None,
        }
    }

    /// Make sure a Node for segment exists in array and return it.
    /// Auto-create either if they don't exist. Entries in array are
    /// sorted by their segment strings.
    fn ensure_node_in_array<'a>(
        array: &mut Option<Vec<SortedPattern>>,
        segment: &'a AuthzRuleSegment,
    ) -> &'a mut Node {
        // Auto-create the array if it doesn't exist
        if array.is_none() {
            *array = Some(Vec::with_capacity(4));
        }

        // Find the node in the array and the index at which it should be inserted
        let array = array.as_mut().unwrap();

        // Try to find an existing node with this segment
        let idx = match array
            .binary_search_by(|element| element.node.segment.as_str().cmp(segment.pattern.as_str()))
        {
            Ok(index) => {
                // Found existing node - return it
                return &mut array[index].node;
            }
            Err(index) => index, // This is where we would insert the new node
        };

        // There is no such node yet.
        // Create one and insert it into the sorted array.
        let mut entry = SortedPattern {
            node: Arc::new(Self::create_node(segment)),
        };

        // Insert at the calculated position to maintain sorted order
        array.push(entry);

        // Return a reference to the newly created node
        &mut array[idx].node
    }
}

/// Since prefix arrays may have more than one hit, we need to link them
/// for fast lookup.
///
/// `sorted_pattern_t`
#[derive(Debug)]
pub struct SortedPattern {
    /// The filtered tree node carrying the prefix.
    node: Arc<Node>,
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
    ///
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

const NO_SEQUENCE_NUMBER: i32 = -1;

impl Default for LimitedRights {
    /// Initialize a limited rights structure.
    /// The minimum rights start with all available access and are later
    /// bitwise-and'ed with actual access rights. The maximum rights begin
    /// empty and are later bitwise-and'ed with actual rights.
    ///
    /// `init_limited_rights`
    fn default() -> Self {
        Self {
            access: PathAccess {
                sequence_number: NO_SEQUENCE_NUMBER,
                rights: AuthzAccess::NONE,
            },
            min_rights: AuthzAccess::WRITE,
            max_rights: AuthzAccess::NONE,
        }
    }
}

impl LimitedRights {
    /// Return TRUE, if RIGHTS has local rights defined in the ACCESS member.
    ///
    /// `has_local_rights`
    pub fn has_local_rights(&self) -> bool {
        self.access.sequence_number != 0
    }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthzRights {
    /// The lowest level of access that the user has to every
    /// path in the repository.
    pub min_access: AuthzAccess,
    /// The highest level of access that the user has to
    /// any path in the repository
    pub max_access: AuthzAccess,
}

impl Default for AuthzRights {
    /// Initialize a rights structure.
    /// The minimum rights start with all available access and are later
    /// bitwise-and'ed with actual access rights. The maximum rights begin
    /// empty and are later bitwise-and'ed with actual rights.
    ///
    /// `init_rights`
    fn default() -> Self {
        Self {
            min_access: AuthzAccess::WRITE,
            max_access: AuthzAccess::NONE,
        }
    }
}

impl AuthzRights {
    /// Set *RIGHTS_P to the combination of LHS and RHS, i.e. intersect the
    /// minimal rights and join the maximum rights.
    ///
    /// `combine_rights`
    pub fn combine(lhs: &Self, rhs: &Self) -> Self {
        Self {
            min_access: lhs.min_access & rhs.min_access,
            max_access: lhs.max_access | rhs.max_access,
        }
    }
}

/// Accumulated global rights for a specific user.
///
/// `authz_global_rights_t`
#[derive(Debug)]
pub struct AuthzGlobalRights {
    /// The user name
    user: String,
    /// Accumulated rights for this user from rules that are not
    /// repository-specific. We use this to avoid a hash lookup for the
    /// "any" repository rights.
    any_reps_rights: AuthzRights,
    /// Accumulated rights for this user across all repositories.
    all_reps_rights: AuthzRights,
    /// Accumulated rights for specific repositories.
    /// The key is repository name, the value is an authz_rights_t*.
    per_repos_rights: HashMap<String, AuthzRights>,
}

impl AuthzGlobalRights {
    /// Initialize a global rights structure.
    /// The USER string must be interned or statically initialized.
    ///
    /// `init_global_rights`
    pub fn new(user: &str) -> Self {
        Self {
            user: user.to_string(),
            any_reps_rights: AuthzRights::default(),
            all_reps_rights: AuthzRights::default(),
            per_repos_rights: HashMap::new(),
        }
    }

    /// Given GLOBAL_RIGHTS and a repository name REPOS, set *RIGHTS_P to
    /// to the actual accumulated rights defined for that repository.
    /// Return TRUE if these rights were defined explicitly.
    ///
    /// `resolve_global_rights`
    pub fn resolve_global_rights(&self, rights: &mut AuthzRights, repos: &str) -> bool {
        if repos.is_empty() {
            /* Return the accumulated rights that are not repository-specific. */
            *rights = self.any_reps_rights;
            return true;
        } else {
            /* Check if we have explicit rights for this repository. */
            if let Some(r) = self.per_repos_rights.get(repos) {
                *rights = AuthzRights::combine(r, &self.any_reps_rights);
                return true;
            }
        }

        // Fall-through: return the rights defined for "any" repository
        // because this user has no specific rules for this specific REPOS.
        *rights = self.all_reps_rights;
        false
    }
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
    /// If PREFIX is indeed a prefix (or exact match) or SEGMENT, add the
    /// node in PREFIX to STATE.
    pub fn add_if_prefix_matches(&mut self, prefix: &SortedPattern, segment: &str) {
        let node = &prefix.node;
        if node.segment.len() <= segment.len() && node.segment.starts_with(segment) {
            self.add_next_node(Some(node.clone()));
        }
    }

    /// Scan the PREFIXES array of node_t* for all entries whose SEGMENT members
    ///  are prefixes of SEGMENT.  Add these to STATE for the next tree level.
    ///
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
///
/// `authz_rule_segment_t`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ///  The path segment must begin with the literal prefix.
    Prefix,

    /// A suffix match: '*' followed by a literal string.
    ///  The path segment must end with the literal suffix.
    ///  The pattern is stored reversed, so that the matching code can
    ///  perform a prefix match on the reversed path segment.
    Suffix,
    /// '*'
    ///  Matches any single non-empty path segment.
    ///  The pattern will be an empty string.
    AnySegment,

    /// '**'
    /// Matches any sequence of zero or more path segments.
    /// The pattern will be an empty string.
    AnyRecursive,
    /// Any other glob/fnmatch pattern.
    Fnmatch,
}

/// Rule path segment descriptor.
///
/// `authz_rule_segment_t`
#[derive(Debug)]
pub struct AuthzRuleSegment {
    pub kind: AuthzRuleSegmentKind,
    pub pattern: String,
}

/// Rule path descriptor.
///
/// `authz_rule_t`
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug, Default)]
pub struct AuthzFull {
    /// All ACLs from the authz file, in the order of definition.
    acls: Vec<AuthzAcl>,

    /// Globally accumulated rights for anonymous access.
    anon_rights: Option<AuthzGlobalRights>,

    /// Globally accumulated rights for authenticated users.
    anthn_rights: Option<AuthzGlobalRights>,

    /// Globally accumulated rights from inverted selectors.
    neg_rights: Option<AuthzGlobalRights>,

    /// Globally accumulated rights, for all concrete users mentioned
    /// in the authz file. The key is the user name, the value is
    /// an authz_global_rights_t*.
    user_rights: HashMap<String, AuthzGlobalRights>,
}

impl AuthzFull {
    /// `has_anon_rights`
    pub fn has_anon_rights(&self) -> bool {
        self.anon_rights.is_some()
    }

    /// `has_authn_rights`
    pub fn has_authn_rights(&self) -> bool {
        self.anthn_rights.is_some()
    }

    /// `has_neg_rights`
    pub fn has_neg_rights(&self) -> bool {
        self.neg_rights.is_some()
    }
    /// Set *RIGHTS to the accumulated global access rights calculated in
    /// AUTHZ for (USER, REPOS).
    /// Return TRUE if the rights are explicit (i.e., an ACL for REPOS
    /// applies to USER, or REPOS is AUTHZ_ANY_REPOSITORY).
    ///
    /// `svn_authz__get_global_rights`
    pub fn get_global_rights(
        &self,
        user: Option<&str>,
        repos: &str,
        rights: &mut AuthzRights,
    ) -> bool {
        const AUTHZ_ANONYMOUS_USER: &str = "";

        match user {
            None | Some(AUTHZ_ANONYMOUS_USER) => {
                // Check if we have explicit rights for anonymous access.
                if let Some(anon_rights) = &self.anon_rights {
                    return anon_rights.resolve_global_rights(rights, repos);
                } else {
                    // No explicit rights for anonymous users, return the
                    // default no-access rights.
                    *rights = AuthzRights::default();
                    return false;
                }
            }
            Some(user_name) => {
                let mut combine_user_rights = false;
                let mut access = false;

                // check if we have explicit rights for this user
                let user_rights = self.user_rights.get(user_name);

                match user_rights {
                    Some(the_rights) => {
                        access = the_rights.resolve_global_rights(rights, repos);
                    }
                    None => {
                        if let Some(neg_rights) = &self.neg_rights {
                            // check if inverted-rule rights apply
                            access = neg_rights.resolve_global_rights(rights, repos);
                            combine_user_rights = true;
                        }
                    }
                }

                // Rights given to _any_ authenticated user may apply, too.
                if let Some(authn_rights) = &self.anthn_rights {
                    let mut authn = AuthzRights::default();
                    let _access = authn_rights.resolve_global_rights(&mut authn, repos);
                    access |= _access;
                    if combine_user_rights {
                        *rights = AuthzRights::combine(&rights, &authn);
                    } else {
                        *rights = authn;
                    }
                }

                access
            }
        }
    }
}

/// Dynamic authorization info
///
/// `svn_authz_t`
pub struct SvnAuthz {
    /// The parsed and pre-processed contents of the authz file.
    full: AuthzFull,

    /// Identifies the authz model content
    /// (a hash value that can be used for e.g. cache lookups).
    authz_id: Bytes,

    /// Rules filtered for a particular user-repository combination.
    /// May be NULL.
    filtered: Option<Arc<AuthzUserRules>>,
}
