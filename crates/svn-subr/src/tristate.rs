/** Generic three-state property to represent an unknown value for values
 * that are just like booleans.  The values have been set deliberately to
 * make tristates disjoint from #svn_boolean_t.
 *
 * @note It is unsafe to use apr_pcalloc() to allocate these, since '0' is
 * not a valid value.
 *
 * @since New in 1.7. */
/// `svn_tristate_t`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tristate {
    /// state known to be false (the constant does not evaluate to false)
    False = 2,
    /// state known to be true
    True,
    /// state could be true or false
    Unknown,
}

impl Tristate {
    /// Return the appropriate tristate for @a word. If @a word is "true", returns
    ///  * #svn_tristate_true; if @a word is "false", returns #svn_tristate_false,
    ///  * for all other values (including NULL) returns #svn_tristate_unknown.
    ///  *
    ///  * @since New in 1.7.
    /// `svn_tristate__from_word`
    pub fn from_str(s: &str) -> Self {
        match s {
            "false" | "no" | "off" | "0" => Tristate::False,
            "true" | "yes" | "on" | "1" => Tristate::True,
            _ => Tristate::Unknown,
        }
    }

    /// Return a constant string "true", "false" or NULL representing the value of
    ///  * @a tristate.
    ///  *
    ///  * @since New in 1.7.
    /// `svn_tristate__to_word`
    pub fn to_str(&self) -> &str {
        match self {
            Tristate::False => "false",
            Tristate::True => "true",
            Tristate::Unknown => "unknown",
        }
    }
}
