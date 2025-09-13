/// Capitalize changing the first letter of each word
pub trait Capitalize {
    /// return the capitalized string
    fn capitalize(&self) -> String;
}

impl Capitalize for String {
    fn capitalize(&self) -> String {
        capitalize_words(self)
    }
}

impl Capitalize for &str {
    fn capitalize(&self) -> String {
        capitalize_words(self)
    }
}

/// Capitalize changing the first letter of each word.
/// No other letters are changed.
///
/// Return the capitalized String
fn capitalize_words(value: &str) -> String {
    let mut result = String::new();
    let mut first = true;

    for value in value.trim().to_lowercase().chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
            if value == ' ' {
                first = true;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::utils::capitalizer::Capitalize;

    #[test]
    fn it_capitalize_lowercase_words() {
        let expected = "Your Ideas, Your World";

        let to_capitalize = "your ideas, your world";
        let result = to_capitalize.capitalize();
        assert_eq!(result, expected);

        let to_capitalize = String::from("your ideas, your world");
        let result = to_capitalize.capitalize();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_capitalize_uppercase_words() {
        let to_capitalize = "YoUR iDeas, yOuR WoRlD";
        let expected = "Your Ideas, Your World";

        let result = to_capitalize.capitalize();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_trim_spaces() {
        let to_capitalize = " YoUR iDeas, yOuR WoRlD ";
        let expected = "Your Ideas, Your World";

        let result = to_capitalize.capitalize();
        assert_eq!(result, expected)
    }
}
