use super::rules::{END, START};

pub static BEFORE_TOKENS: &[char] = &['(', START.as_bytes()[0] as char, END.as_bytes()[0] as char];
pub static AFTER_TOKENS: &[char] = &[
    '.',
    '!',
    '?',
    ',',
    ':',
    ')',
    START.as_bytes()[0] as char,
    END.as_bytes()[0] as char,
];

/// Replaces any occurrences of the given char twice in a row with the char once (removes double spaces for example)
fn remove_double_char(mut string: String, char: char) -> String {
    let double: String = [char, char].into_iter().collect();
    let char_str = char.to_string();

    while string.contains(&double) {
        string = string.replace(&double, &char_str);
    }

    string
}

/// Merges the vector into a single string
pub fn merge(vector: Vec<String>) -> String {
    let mut result = vector.join(" ");

    for token in BEFORE_TOKENS {
        let before_string = format!("{} ", &token.to_string().as_str());
        result = result.replace(&before_string, &token.to_string().as_str());
    }

    for token in AFTER_TOKENS {
        let after_string = format!(" {}", &token.to_string().as_str());
        result = result.replace(&after_string, &token.to_string().as_str());
    }

    result = result.replace(START, "");
    result = result.replace(END, "\n");

    // Removes double new lines
    result = remove_double_char(result, '\n');
    // Removes double spaces
    result = remove_double_char(result, ' ');

    result
}

#[cfg(test)]
mod tests {
    use crate::wfc::wfc::{
        post_processing::merge,
        rules::{END, START},
    };

    use super::remove_double_char;

    #[test]
    fn test_remove_double_char_sanity() {
        let string = "hello          there".to_string();
        let result = remove_double_char(string, ' ');

        assert_eq!(result, "hello there".to_string());
    }

    #[test]
    fn test_merge_sanity() {
        let vector = vec![
            START.to_string(),
            "hello".to_string(),
            "world".to_string(),
            END.to_string(),
        ];

        assert_eq!(merge(vector), "hello world\n".to_string());
    }

    #[test]
    fn test_merge_tokens() {
        let vector = vec![
            START.to_string(),
            "hello".to_string(),
            "world".to_string(),
            "!".to_string(),
            "is".to_string(),
            "this".to_string(),
            "(".to_string(),
            "thing".to_string(),
            ")".to_string(),
            ",".to_string(),
            "working".to_string(),
            "?".to_string(),
            END.to_string(),
        ];

        assert_eq!(
            merge(vector),
            "hello world! is this (thing), working?\n".to_string()
        );
    }

    #[test]
    fn test_merge_multiline() {
        let vector = vec![
            START.to_string(),
            "hello".to_string(),
            "world".to_string(),
            "!".to_string(),
            END.to_string(),
            START.to_string(),
            "is".to_string(),
            "this".to_string(),
            "(".to_string(),
            "thing".to_string(),
            ")".to_string(),
            ",".to_string(),
            "working".to_string(),
            "?".to_string(),
            END.to_string(),
        ];

        assert_eq!(
            merge(vector),
            "hello world!\nis this (thing), working?\n".to_string()
        );
    }
}
