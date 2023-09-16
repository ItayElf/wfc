use crate::wfc::wfc::rules::Rules;

/// Parses a json string as a rules type type
fn parse_rules_json(json_string: &str) -> Result<Rules, serde_json::Error> {
    serde_json::from_str(json_string)
}

/// Returns a string object of the serialized rules
fn rules_to_json(rules: &Rules) -> Result<String, serde_json::Error> {
    serde_json::to_string(rules)
}

#[cfg(test)]
mod tests {

    use crate::wfc::wfc::rules::{Allowed, Rules, END, START};

    use super::{parse_rules_json, rules_to_json};

    #[test]
    fn test_parse_rules_json_sanity() {
        let string = r#"
            {
                "hello": {
                    "after": ["world", "there"],
                    "before": ["\u0002"]
                },
                "\u0002": {
                    "before": ["\u0003"],
                    "after": ["world"]
                }
            }
        "#;

        let result: Rules = parse_rules_json(string).unwrap();

        assert!(result.contains_key(&"hello".to_string()));
        assert!(result.contains_key(&START.to_string()));
        assert_eq!(result.keys().len(), 2);
        assert_eq!(result["hello"].after.len(), 2);
        assert!(result["hello"].before.contains(&START.to_string()));
    }

    #[test]
    fn test_parse_rules_json_missing_fields() {
        let string = r#"
            {
                "hello": {
                    "after": ["world", "there"]
                },
                "\u0002": {
                    "after": ["world"]
                }
            }
        "#;

        let result = parse_rules_json(string).err().unwrap();
        assert!(result.to_string().contains("missing field `before`"));
    }

    #[test]
    fn test_rules_to_json_sanity() {
        let mut rules = Rules::new();
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
            START.to_string(),
            Allowed::new(
                [END.to_string()].into_iter().collect(),
                ["hello".to_string()].into_iter().collect(),
            ),
        );
        let string = rules_to_json(&rules).unwrap();
        let result = parse_rules_json(&string).unwrap();

        assert!(result.contains_key(&"hello".to_string()));
        assert!(result.contains_key(&START.to_string()));
        assert_eq!(result.keys().len(), 2);
        assert_eq!(result["hello"].after.len(), 2);
        assert!(result["hello"].before.contains(&START.to_string()));
    }
}
