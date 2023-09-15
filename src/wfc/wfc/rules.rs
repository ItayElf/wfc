use std::{
    collections::{HashMap, HashSet},
    vec,
};

/// A type representing the rules by which the algorithm works (A map of word to its allowed words).
pub type Rules = HashMap<String, Allowed>;

/// A type for the vector the algorithm work on
pub type WfcVector = Vec<HashSet<String>>;

/// Represents start of line
pub static START: &str = "\x02";
/// Represents end of line
pub static END: &str = "\x03";

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

pub fn generate_wfc_vector(rules: &Rules, size: usize) -> WfcVector {
    let mut vector = WfcVector::with_capacity(size + 2);
    let set: HashSet<String> = rules.keys().into_iter().cloned().collect();

    for _i in 0..(size + 2) {
        vector.push(set.clone())
    }

    vector[0] = [START.to_string()].into_iter().collect();
    vector[size + 1] = [END.to_string()].into_iter().collect();

    vector
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{generate_wfc_vector, Allowed, Rules, END, START};

    fn get_rules() -> Rules {
        let mut rules = HashMap::<String, Allowed>::new();
        rules.insert(
            START.to_string(),
            Allowed::new(
                [END.to_string()].into_iter().collect(),
                ["hello".to_string()].into_iter().collect(),
            ),
        );
        rules.insert(
            "hello".to_string(),
            Allowed::new(
                [START.to_string()].into_iter().collect(),
                ["world".to_string(), "there".to_string()]
                    .into_iter()
                    .collect(),
            ),
        );

        rules.insert(
            "world".to_string(),
            Allowed::new(
                ["hello".to_string()].into_iter().collect(),
                ["!".to_string()].into_iter().collect(),
            ),
        );

        rules.insert(
            "there".to_string(),
            Allowed::new(
                ["hello".to_string()].into_iter().collect(),
                [END.to_string()].into_iter().collect(),
            ),
        );

        rules.insert(
            "!".to_string(),
            Allowed::new(
                ["world".to_string()].into_iter().collect(),
                [END.to_string()].into_iter().collect(),
            ),
        );

        rules.insert(
            END.to_string(),
            Allowed::new(
                ["there".to_string()].into_iter().collect(),
                [START.to_string()].into_iter().collect(),
            ),
        );

        rules
    }

    #[test]
    fn test_generate_wfc_vector_sanity() {
        let size = 10usize;
        let rules = get_rules();
        let vector = generate_wfc_vector(&rules, size);

        assert_eq!(vector.capacity(), size + 2);
        assert_eq!(vector.len(), size + 2);
        assert_eq!(vector[0].iter().next().unwrap(), &START.to_string());
        assert_eq!(vector[size + 1].iter().next().unwrap(), &END.to_string());
        assert_eq!(vector[1].len(), rules.keys().len());
    }
}
