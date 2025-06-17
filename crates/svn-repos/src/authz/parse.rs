//! Parser for path-base access control
//!
//! `authz_parse.c`

use std::collections::HashMap;

use crate::authz::AuthzFull;

use super::AuthzAcl;

/// Temporary ACL constructed by the parser.
///
/// `parsed_acl_t`
#[derive(Debug)]
pub struct ParsedAcl {
    /// The global ACL.
    /// The strings in ACL.rule are allocated from the result pool.
    /// ACL.user_access is null during the parsing stage.
    acl: AuthzAcl,
    /// The set of access control entries. In the second pass, aliases in
    /// these entries will be expanded and equivalent entries will be
    /// merged. The entries are allocated from the parser pool.
    aces: HashMap<String, AccessType>,
    /// The set of access control entries that use aliases. In the second
    /// pass, aliases in these entries will be expanded and merged into ACES.
    /// The entries are allocated from the parser pool.
    alias_aces: HashMap<String, AccessType>,
}

/// Temporary group definition constructed by the authz/group parser.
/// Once all groups and aliases are defined, a second pass over these
/// data will recursively expand group memberships.
///
/// `parsed_group_t`
pub struct ParsedGroup {
    local_group: bool,
    members: Vec<String>,
}

/// An empty string with a known address.
const INTERNED_EMPTY_STRING: &str = "";
/// The name of the aliases section.
const ALIASES_SECTION: &str = "aliases";
/// The name of the groups section.
const GROUPS_SECTION: &str = "groups";
/// The token indicating that an authz rule contains wildcards.
const GLOB_RULE_TOKEN: &str = "glob";
/// The anonymous access token.
const ANON_ACCESS_TOKEN: &str = "$anonymous";
/// The authenticated access token.
const AUTHN_ACCESS_TOKEN: &str = "$authenticated";
/// Fake token for inverted rights.
const NEG_ACCESS_TOKEN: &str = "~~$inverted";

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Section = HashMap<String, String>;

use chumsky::prelude::*;

#[derive(Debug)]
pub struct AuthzParser {
    global: HashMap<String, String>,
    sections: HashMap<String, Section>,
}

impl AuthzParser {
    pub fn parse_file<P>(file: P) -> Result<AuthzFull, ParseError>
    where
        P: AsRef<std::path::Path>,
    {
        let input = fs_err::fs::read_to_string(file)?;
        Self::parse(&input)
    }

    pub fn parse(input: &str) -> Result<AuthzFull, ParseError> {
        let mut authz_full = AuthzFull::default();
        Ok(authz_full)
    }

    fn _parse<'a>() -> Parser<'a, &'a str, Self, extra::Err<Rich<'a, char>>> {
        recursive(|value| {
            // let
            todo!()
            // choice((
            //     just()
            // ))
        })
    }
}
