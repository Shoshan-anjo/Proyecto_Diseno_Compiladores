use std::collections::HashMap;

pub fn validate_and_translate(code: Vec<String>) -> Vec<char> {
    let morse_map = get_morse_map();

    code.iter()
        .filter_map(|morse| {
            morse_map.get(morse).copied().or_else(|| {
                eprintln!("⚠️  Morse code '{}' not recognized, using '?'", morse);
                Some('?')
            })
        })
        .collect()
}

fn get_morse_map() -> HashMap<String, char> {
    let mut map = HashMap::new();
    map.insert("...".to_string(), 'S');
    map.insert("---".to_string(), 'O');
    map.insert(".-".to_string(), 'A');
    map.insert("-.-".to_string(), 'K');
    map.insert("-.-.".to_string(), 'C');
    map.insert("-..".to_string(), 'B');
    map.insert(".-..".to_string(), 'L');
    map.insert("--".to_string(), 'M');
    map.insert("-.".to_string(), 'N');
    map.insert("--.".to_string(), 'G');
    map.insert("....".to_string(), 'E');
    map.insert("..--".to_string(), 'F');
    map.insert("--.-".to_string(), 'Q');
    map.insert("...-".to_string(), 'V');
    map.insert(".--".to_string(), 'W');
    map.insert("-.--".to_string(), 'Y');
    map.insert("--..".to_string(), 'Z');
    map.insert(".----".to_string(), '1');
    map.insert("..---".to_string(), '2');
    map.insert("...--".to_string(), '3');
    map.insert("....-".to_string(), '4');
    map.insert(".....".to_string(), '5');
    map.insert("-....".to_string(), '6');
    map.insert("--...".to_string(), '7');
    map.insert("---..".to_string(), '8');
    map.insert("----.".to_string(), '9');
    map.insert("-----".to_string(), '0');
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morse_to_letter() {
        let result = validate_and_translate(vec!["...".to_string(), "---".to_string()]);
        assert_eq!(result, vec!['S', 'O']);
    }

    #[test]
    fn test_unknown_morse_code() {
        let result = validate_and_translate(vec!["....".to_string()]);
        assert_eq!(result, vec!['E']);
    }
}
