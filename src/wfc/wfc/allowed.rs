use std::collections::{HashMap, HashSet};

/// A type representing the rules by which the algorithm works (A map of word to its allowed words).
pub type Rules = HashMap<String, Allowed>;

/// This struct holds the set of words which are allowed to appear after and before a given word.
#[derive(Clone)]
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
