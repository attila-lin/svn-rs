use svn_repos::AuthzParser;
use svn_repos::authz::AuthzAccess;

// `test_global_rights`
#[rstest]
// Everyone may get read access b/c there might be a "/public" path.
#[case("", "", { AuthzAccess::None, AuthzAccess::Read }, true)]
#[case("", "userA", { AuthzAccess::None, AuthzAccess::Read }, true)]
#[case("", "userB", { AuthzAccess::None, AuthzAccess::Read }, true)]
#[case("", "userC", { AuthzAccess::None, AuthzAccess::Read }, true)]
/* Two users do even get write access on some paths in "greek".
 * The root always defaults to n/a due to the default rule. */
#[case("greek", "", { AuthzAccess::None, AuthzAccess:Read }, false)]
#[case("greek", "userA", { AuthzAccess::None, AuthzAccess::Write }, true)]
#[case("greek", "userB", { AuthzAccess::None, AuthzAccess::Write }, true)]
#[case("greek", "userC", { AuthzAccess::None, AuthzAccess::Read }, false)]
/* One users has write access to some paths in "repo". */
#[case("repo", "", { AuthzAccess::None, AuthzAccess::Read }, false)]
#[case("repo", "userA", { AuthzAccess::None, AuthzAccess::Write }, true)]
#[case("repo", "userB", { AuthzAccess::None, AuthzAccess::Read }, false)]
#[case("repo", "userC", { AuthzAccess::None, AuthzAccess::Read }, false)]
/* For unknown repos, we default to the global settings. */
#[case("X", "", { AuthzAccess::None, AuthzAccess::Read }, false)]
#[case("X", "userA", { AuthzAccess::None, AuthzAccess::Read }, false)]
#[case("X", "userB", { AuthzAccess::None, AuthzAccess::Read }, false)]
#[case("X", "userC", { AuthzAccess::None, AuthzAccess::Read }, false)]
fn test_global_rights_1(
    #[case] path: &str,
    #[case] user: &str,
    #[case] expected: (AuthzAccess, AuthzAccess),
    #[case] found: bool,
) {
    let authz1 = r#"
[/public]
* = r

[greek:/A]
userA = rw

[repo:/A]
userA = r

[repo:/B]
userA = rw

[greek:/B]
userB = rw
        "#;
    let parser = AuthzParser::parse(authz1).unwrap();
}

#[test]
fn test_global_rights_2() {
    let authz2 = r#"
[/]
userA = r

[/public]
userB = rw

[repo:/]
userA = rw
"#;
    let parser = AuthzParser::parse(authz2).unwrap();
}
