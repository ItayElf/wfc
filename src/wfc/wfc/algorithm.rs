use std::collections::HashSet;

use rand::seq::SliceRandom;

type WfcVector = Vec<HashSet<String>>;

/// Returns wether the vector is collapsed
pub fn is_collapsed(wfc_vector: &WfcVector) -> bool {
    for set in wfc_vector {
        if set.len() != 1 {
            return false;
        }
    }
    true
}

/// Collapse the set at the given location, returning the value
fn collapse_at(wfc_vector: &mut WfcVector, index: usize) -> Result<String, &'static str> {
    if index >= wfc_vector.len() {
        return Err("Index was out of range");
    }
    let set = &wfc_vector[index];
    let set_as_vec: Vec<&String> = set.iter().collect();

    let value = set_as_vec.choose(&mut rand::thread_rng());

    if value.is_none() {
        return Err("Set was empty");
    }

    let value = String::from(*value.unwrap());
    wfc_vector[index] = vec![value.clone()].into_iter().collect();

    Ok(value)
}

/// Converts a WfcVector to a vector of strings.
pub fn flatten_wfc_vector(wfc_vector: WfcVector) -> Result<Vec<String>, &'static str> {
    let mut result = Vec::<String>::new();

    for set in wfc_vector {
        if set.len() != 1 {
            return Err("Set has more than one string!");
        }
        result.push(set.into_iter().next().unwrap());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::wfc::wfc::algorithm::{collapse_at, is_collapsed};

    use super::{flatten_wfc_vector, WfcVector};

    #[test]
    fn test_is_collapsed_sanity() {
        let mut vector: WfcVector = vec![
            vec![String::from("hello")].into_iter().collect(),
            vec![String::from("world")].into_iter().collect(),
        ];

        assert_eq!(is_collapsed(&vector), true);

        vector[0].insert(String::new());

        assert_eq!(is_collapsed(&vector), false);
    }

    #[test]
    fn test_collapse_at_sanity() {
        let mut vector: WfcVector = vec![
            vec![
                String::from("hello"),
                String::from("world"),
                String::from("test"),
            ]
            .into_iter()
            .collect(),
            vec![String::from("world")].into_iter().collect(),
        ];
        assert_ne!(vector[0].len(), 1);

        collapse_at(&mut vector, 0).unwrap();

        assert_eq!(vector[0].len(), 1);
    }

    #[test]
    fn test_collapse_at_empty_set() {
        let mut vector: WfcVector = vec![vec![].into_iter().collect()];

        let result = collapse_at(&mut vector, 0);

        assert_eq!(result, Err("Set was empty"));
    }

    #[test]
    fn test_collapse_at_invalid_index() {
        let mut vector: WfcVector = vec![];

        let result = collapse_at(&mut vector, 100);

        assert_eq!(result, Err("Index was out of range"));
    }

    #[test]
    fn test_flatten_wfc_vector_sanity() {
        let vector: WfcVector = vec![
            vec![String::from("hello")].into_iter().collect(),
            vec![String::from("world")].into_iter().collect(),
        ];
        let result: Vec<String> = vec![String::from("hello"), String::from("world")];

        assert_eq!(flatten_wfc_vector(vector), Ok(result))
    }

    #[test]
    fn test_flatten_wfc_vector_error() {
        let vector: WfcVector = vec![
            vec![
                String::from("hello"),
                String::from("world"),
                String::from("test"),
            ]
            .into_iter()
            .collect(),
            vec![String::from("world")].into_iter().collect(),
        ];

        let result = flatten_wfc_vector(vector);
        assert_eq!(result, Err("Set has more than one string!"))
    }
}
