use crate::wfc::wfc::rules::Rules;

/// Parses a json string as a rules type type
fn parse_rules_json(json_string: &str) -> Result<Rules, serde_json::Error> {
    serde_json::from_str(json_string)
}

#[cfg(test)]
mod tests {

    use crate::wfc::wfc::rules::{Rules, START};

    use super::parse_rules_json;

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
}
