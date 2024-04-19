pub fn tidy_message(message: &str, strict_ascii: bool) -> String {
    let mut result = message.to_lowercase();
    if strict_ascii {
        let mut index = 0;
        for char in result.clone().chars() {
            if !char.is_ascii() {
                result.remove(index);
            } else {
                index += 1;
            }
        }
    }
    result.trim().to_owned()
}

pub fn contains_profanity(message: &str, strict_ascii: bool) -> bool {
    let message = tidy_message(message, strict_ascii);
    let profanity = include_str!("Profanity.csv").split(',').collect::<Vec<&str>>();
    for profanity_word in profanity {
        for message_word in message.split(' ') {
            if message_word == profanity_word {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(contains_profanity("piss", false), true);
        assert_eq!(contains_profanity("Hello, World!", false), false);
    }

    #[test]
    fn test_tidy_relaxed() {
        assert_eq!(tidy_message("  Hello ", false), "hello");
        assert_eq!(tidy_message("  Hello ", false), "hello");
    }

    #[test]
    fn test_tidy_strict() {
        assert_eq!(tidy_message("🗿 Hello", true), "hello");
        assert_eq!(tidy_message("🗿 Hello", true), "hello");
    }
}