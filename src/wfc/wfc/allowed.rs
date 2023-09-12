use std::collections::HashSet;

/// This struct holds the set of words which are allowed to appear after and before a given word.
pub struct Allowed {
    pub before: HashSet<String>,
    pub after: HashSet<String>,
}

impl Allowed {
    /// Creates a new allowed struct.
    pub const fn new(before: HashSet<String>, after: HashSet<String>) -> Self {
        Self { before, after }
    }
}
