/// Our internal representation of a change
///
/// `binary_change_t`
struct BinaryChange {
    /// define the kind of change and what specific information is present
    flags: i32,
    /// Path of the change.
    path: usize,
}
