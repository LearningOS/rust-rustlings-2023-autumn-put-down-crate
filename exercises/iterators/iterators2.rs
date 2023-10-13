// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut new_word:Vec<char> = vec![];
    
    loop {
        match c.next() {
            None => break,
            Some(first) => {
                if new_word.len() == 0 {
                    new_word.push(first.clone().to_ascii_uppercase());
                    continue;
                }
                new_word.push(first.clone());
            },
        }    
    }
    let real_word = new_word.iter().collect::<String>();
    real_word
    
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut new_words:Vec<String>=vec![];
    for w in words {
        let new_word=capitalize_first(*w);
        new_words.push(new_word);
    }
    new_words
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut new_chars: Vec<char>=vec![];
    for old_word in words {
        let mut old_chars = old_word.chars();
        loop {
            match old_chars.next() {
                None => break,
                Some(old_char) => {
                    if (new_chars.len() == 0 && old_char != ' ') || (new_chars.last() == Some(&' ') && old_char != ' ') {
                        new_chars.push(old_char.clone().to_ascii_uppercase());
                        continue;
                    }
                    new_chars.push(old_char.clone());
                }
            }
        }
    }
    let new_word = new_chars.iter().collect::<String>();
    new_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
