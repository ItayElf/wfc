use std::collections::HashSet;

use rand::seq::SliceRandom;

use super::allowed::Rules;

type WfcVector = Vec<HashSet<String>>;

static PLACEHOLDER: String = String::new();
static START: &str = "\x02";
static END: &str = "\x03";

/// Returns the left index (before)
fn get_left_neighbor(index: usize) -> Option<usize> {
    if index > 0 {
        return Some(index - 1);
    }
    None
}

/// Returns the right index (after)
fn get_right_neighbor(index: usize, max_length: usize) -> Option<usize> {
    if index < (max_length - 1) {
        return Some(index + 1);
    }
    None
}

/// Returns valid neighbor indexes
fn get_valid_neighbors(index: usize, max_length: usize) -> impl Iterator<Item = usize> {
    let mut result = Vec::<usize>::new();
    if index > 0 {
        result.push(index - 1);
    }
    if index < (max_length - 1) {
        result.push(index + 1);
    }
    result.into_iter()
}

/// Returns wether the vector is collapsed
fn is_collapsed(wfc_vector: &WfcVector) -> bool {
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

/// Returns a set with the possible values based on neighbors
fn get_valid_options_from_neighbors(
    wfc_vector: &WfcVector,
    rules: &Rules,
    index: usize,
) -> HashSet<String> {
    let mut result = wfc_vector[index].clone();
    let before = get_left_neighbor(index);
    let after = get_right_neighbor(index, wfc_vector.len());

    if before.is_some() {
        let before = before.unwrap();
        let set = &wfc_vector[before];
        let mut sum = HashSet::<String>::new();
        for word in set {
            let after_rules = &rules[word].after;
            sum.extend(after_rules.iter().map(|s| s.to_string()));
        }
        result = result.intersection(&sum).map(|s| s.to_string()).collect();
    }

    if after.is_some() {
        let after = after.unwrap();
        let set = &wfc_vector[after];
        let mut sum = HashSet::<String>::new();
        for word in set {
            let before_rules = &rules[word].before;
            sum.extend(before_rules.iter().map(|s| s.to_string()));
        }
        result = result.intersection(&sum).map(|s| s.to_string()).collect();
    }

    if result.is_empty() {
        result.insert(PLACEHOLDER.clone());
    }

    result
}

/// Converts a WfcVector to a vector of strings.
fn flatten_wfc_vector(wfc_vector: WfcVector) -> Result<Vec<String>, &'static str> {
    let mut result = Vec::<String>::new();

    for set in wfc_vector {
        if set.len() != 1 {
            return Err("Set has more than one string!");
        }
        result.push(set.into_iter().next().unwrap());
    }

    Ok(result)
}

/// Limits the possible values based on the last collapsed value
fn propagate(wfc_vector: &mut WfcVector, rules: &Rules, last_collapse_index: usize) {
    let mut stack = Vec::<usize>::new();
    let mut current_index = Some(last_collapse_index);

    while current_index.is_some() {
        let unwrapped_current_index = current_index.unwrap();
        let neighbors = get_valid_neighbors(unwrapped_current_index, wfc_vector.len());

        for neighbor_index in neighbors {
            let set = &wfc_vector[neighbor_index];
            let new_set = get_valid_options_from_neighbors(wfc_vector, rules, neighbor_index);
            let new_set = new_set.iter().map(|s| s.to_string()).collect();

            if !stack.contains(&neighbor_index) & (*set != new_set) {
                stack.push(neighbor_index);
            }

            wfc_vector[neighbor_index] = new_set;
        }

        current_index = stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::wfc::wfc::{
        algorithm::{collapse_at, get_valid_neighbors, is_collapsed, PLACEHOLDER},
        allowed::{Allowed, Rules},
    };

    use super::{
        flatten_wfc_vector, get_valid_options_from_neighbors, propagate, WfcVector, END, START,
    };

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
    fn test_get_valid_neighbors_sanity() {
        assert_eq!(
            get_valid_neighbors(2, 100).into_iter().collect::<Vec<_>>(),
            vec![1, 3]
        );
        assert_eq!(
            get_valid_neighbors(0, 100).into_iter().collect::<Vec<_>>(),
            vec![1]
        );
        assert_eq!(
            get_valid_neighbors(1, 2).into_iter().collect::<Vec<_>>(),
            vec![0]
        );
    }

    #[test]
    fn test_is_collapsed_sanity() {
        let mut vector: WfcVector = vec![
            vec!["hello".to_string()].into_iter().collect(),
            vec!["world".to_string()].into_iter().collect(),
        ];

        assert_eq!(is_collapsed(&vector), true);

        vector[0].insert(String::new());

        assert_eq!(is_collapsed(&vector), false);
    }

    #[test]
    fn test_collapse_at_sanity() {
        let mut vector: WfcVector = vec![
            vec!["hello".to_string(), "world".to_string(), "test".to_string()]
                .into_iter()
                .collect(),
            vec!["world".to_string()].into_iter().collect(),
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
            vec!["hello".to_string()].into_iter().collect(),
            vec!["world".to_string()].into_iter().collect(),
        ];
        let result: Vec<String> = vec!["hello".to_string(), "world".to_string()];

        assert_eq!(flatten_wfc_vector(vector), Ok(result))
    }

    #[test]
    fn test_flatten_wfc_vector_error() {
        let vector: WfcVector = vec![
            vec!["hello".to_string(), "world".to_string(), "test".to_string()]
                .into_iter()
                .collect(),
            vec!["world".to_string()].into_iter().collect(),
        ];

        let result = flatten_wfc_vector(vector);
        assert_eq!(result, Err("Set has more than one string!"))
    }

    #[test]
    fn test_get_valid_options_from_neighbors_sanity() {
        let rules = get_rules();
        let vector: WfcVector = vec![
            vec!["hello".to_string()].into_iter().collect(),
            rules.keys().cloned().into_iter().collect(),
            vec!["!".to_string(), END.to_string()].into_iter().collect(),
        ];
        let result = get_valid_options_from_neighbors(&vector, &rules, 1);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&"world".to_string()));
        assert!(result.contains(&"there".to_string()));
    }

    #[test]
    fn test_get_valid_options_from_neighbors_impossible() {
        let rules = get_rules();
        let vector: WfcVector = vec![
            vec!["!".to_string()].into_iter().collect(),
            rules.keys().cloned().into_iter().collect(),
            vec![END.to_string()].into_iter().collect(),
        ];
        let result = get_valid_options_from_neighbors(&vector, &rules, 1);

        assert_eq!(result.len(), 1);
        assert_eq!(result.into_iter().next().unwrap(), PLACEHOLDER);
    }

    #[test]
    fn test_propagate_sanity() {
        let rules = get_rules();
        let mut vector: WfcVector = vec![
            vec![START.to_string()].into_iter().collect(),
            vec!["hello".to_string()].into_iter().collect(),
            rules.keys().cloned().into_iter().collect(),
            vec!["!".to_string(), END.to_string()].into_iter().collect(),
        ];
        propagate(&mut vector, &rules, 1);

        assert_eq!(
            vector,
            vec![
                vec![START.to_string()].into_iter().collect(),
                vec!["hello".to_string()].into_iter().collect(),
                vec!["world".to_string(), "there".to_string()]
                    .into_iter()
                    .collect(),
                vec!["!".to_string(), END.to_string()].into_iter().collect(),
            ]
        );
    }
}
