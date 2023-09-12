use std::collections::HashSet;

type WfcVector = Vec<HashSet<String>>;

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
    use super::{flatten_wfc_vector, WfcVector};

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
