pub fn abbreviate(phrase: &str) -> String {

    /*
    // Attempt 1
    // TOFIX: only camelcase fails
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            // find the first alphabetic character in the word
            word.chars()
                .find(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_uppercase())
        })
        .flatten()
        .collect()
    */

    // Attempt 2
    // TOFIX: only all_caps_word fails
    let mut acronym = String::new();
    let mut prev: Option<char> = None;

    for c in phrase.chars() {
        if c.is_alphabetic() {
            let is_start = prev.is_none_or(|p| {
                p.is_whitespace()
                || p == '-'
                || (c.is_uppercase() && !p.is_uppercase())
            });
            if is_start {
                acronym.push(c.to_ascii_uppercase());
            }
        }
        prev = Some(c);
    }

    acronym

    /*
    // Attempt 3
    // TOFIX: only underscore_emphasis fails
    let mut acronym = String::new();
    let words = phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter(|word| !word.is_empty());
    for word in words {
        let mut chars = word.chars();
        acronym.push(chars.next().unwrap().to_ascii_uppercase());
        for c in chars {
            if word.chars().all(|c: char| c.is_uppercase()) {
                // only push the first character of the word to acronym
                // already pushed before, so no need to do anything
                break;
            }
            if c.is_uppercase() {
                acronym.push(c.to_ascii_uppercase());
            }
        }
    }

    acronym
    */

    /*
    // Attempt 4
    // TOFIX: 
    let mut acronym = String::new();
    let words = phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter(|word| !word.is_empty());
    for word in words {
        let mut chars = word.chars();
        for c in chars {
            if word.chars().all(|c: char| c.is_uppercase()) {
                // only push the first character of the word to acronym
                acronym.push(c.to_ascii_uppercase());
                break;
            }
            // if c is an alphabet, and c is the first character of the word, or c is uppercase (to handle CamelCase input)
            if c.is_alphabetic() && (c == word.chars().next().unwrap() || c.is_uppercase()) {
                acronym.push(c.to_ascii_uppercase());
            }
        }
    }

    acronym
    */
}
