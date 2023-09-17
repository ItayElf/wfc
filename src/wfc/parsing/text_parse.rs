use crate::wfc::wfc::rules::{END, START};

use super::post_processing::{remove_double_char, AFTER_TOKENS, BEFORE_TOKENS};

/// Returns the given text as an array of tokens
pub fn parse_text(mut text: String) -> Vec<String> {
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

    text = text.replace("\n", " \n ");
    text = text.replace("\n", &(END.to_owned() + " " + START));
    text = format!("{START} {text} {END}");

    text = remove_double_char(text, ' ');
    text = remove_double_char(text, '\n');

    text.split(" ").map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use crate::wfc::{
        parsing::text_parse::parse_text,
        wfc::rules::{END, START},
    };

    #[test]
    fn test_parse_text_sanity() {
        let text = "hello there nice to meet you\n".to_string();
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
        let text = "hello there, nice to meet (you)!\n".to_string();
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
            [
                START, "hello", "there", END, START, "nice", "to", "meet", "(", "you", ")", "!",
                END
            ]
            .map(|s| s.to_string())
            .into_iter()
            .collect::<Vec<_>>()
        )
    }
}
