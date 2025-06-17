use svn_repos::AuthzParser;

// `test_global_rights`
#[test]
fn test_global_rights() {
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
