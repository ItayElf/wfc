use crate::wfc::wfc::{
    algorithm::{get_left_neighbor, get_right_neighbor},
    rules::{Allowed, Rules, END, START},
};

use super::post_processing::{remove_double_char, AFTER_TOKENS, BEFORE_TOKENS};

/// Returns the given text as an array of tokens
fn parse_text(mut text: String) -> Vec<String> {
    text = text.to_lowercase();
    if text.ends_with('\n') {
        text.pop();
    }

    for token in BEFORE_TOKENS {
        let before_string = format!("{} ", &token.to_string().as_str());
        text = text.replace(&token.to_string().as_str(), &before_string);
    }

    for token in AFTER_TOKENS {
        let after_string = format!(" {}", &token.to_string().as_str());
        text = text.replace(&token.to_string().as_str(), &after_string);
    }

    text = remove_double_char(text, ' ');
    text = format!("{START} {text} {END}");

    text.split(" ").map(|s| s.to_string()).collect()
}

/// Converts a vector of tokens into rules object
fn convert_tokens_to_rules(tokens: &Vec<String>) -> Rules {
    let mut rules = Rules::new();

    for (i, token) in tokens.iter().enumerate() {
        if !rules.contains_key(token) {
            rules.insert(token.clone(), Allowed::empty());
        }

        if let Some(allowed) = rules.get_mut(token) {
            if let Some(left) = get_left_neighbor(i) {
                let left_neighbor = tokens[left].clone();
                let before = &mut allowed.before;
                before.insert(left_neighbor);
            }

            if let Some(right) = get_right_neighbor(i, tokens.len()) {
                let right_neighbor = tokens[right].clone();
                let after = &mut allowed.after;
                after.insert(right_neighbor);
            }
        }
    }

    if let Some(end) = rules.get_mut(END) {
        end.after.insert(START.to_owned());
    }
    if let Some(start) = rules.get_mut(START) {
        start.before.insert(END.to_owned());
    }

    rules
}

/// Generates rules from an existing text
pub fn generate_rules(text: String) -> Rules {
    let tokens = parse_text(text);
    convert_tokens_to_rules(&tokens)
}

#[cfg(test)]
mod tests {
    use crate::wfc::{
        parsing::text_parse::parse_text,
        wfc::rules::{END, START},
    };

    use super::convert_tokens_to_rules;

    #[test]
    fn test_parse_text_sanity() {
        let text = "Hello there nice to meet you\n".to_string();
        assert_eq!(
            parse_text(text),
            [START, "hello", "there", "nice", "to", "meet", "you", END]
                .map(|s| s.to_string())
                .into_iter()
                .collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_parse_text_tokens() {
        let text = "Hello there, Nice to meet (you)!\n".to_string();
        assert_eq!(
            parse_text(text),
            [START, "hello", "there", ",", "nice", "to", "meet", "(", "you", ")", "!", END]
                .map(|s| s.to_string())
                .into_iter()
                .collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_parse_text_multiline() {
        let text = "hello there\nnice to meet (you)!".to_string();
        assert_eq!(
            parse_text(text),
            [START, "hello", "there", "\n", "nice", "to", "meet", "(", "you", ")", "!", END]
                .map(|s| s.to_string())
                .into_iter()
                .collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_convert_tokens_to_rules_sanity() {
        let vector = [START, "hello", "there", END]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();

        let rules = convert_tokens_to_rules(&vector);

        assert_eq!(rules.keys().len(), 4);
        assert!(rules.contains_key(END));
        assert!(rules.contains_key(START));
        assert!(rules.contains_key("hello"));
        assert!(rules.contains_key("there"));
        assert!(rules[END].after.contains(START));
        assert!(rules[END].before.contains("there"));
        assert!(rules[START].after.contains("hello"));
        assert!(rules[START].before.contains(END));
        assert!(rules["hello"].after.contains("there"));
        assert!(rules["hello"].before.contains(START));
        assert!(rules["there"].after.contains(END));
        assert!(rules["there"].before.contains("hello"));
    }

    #[test]
    fn test_convert_tokens_to_rules_multiple() {
        let vector = [START, "hello", "there", "\n", "hello", "world", END]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>();

        let rules = convert_tokens_to_rules(&vector);

        assert_eq!(rules.keys().len(), 6);
        assert!(rules.contains_key(END));
        assert!(rules.contains_key(START));
        assert!(rules.contains_key("hello"));
        assert!(rules.contains_key("there"));
        assert!(rules.contains_key("world"));
        assert!(rules.contains_key("\n"));
        assert!(rules["hello"].after.contains("there"));
        assert!(rules["hello"].after.contains("world"));
    }
}
