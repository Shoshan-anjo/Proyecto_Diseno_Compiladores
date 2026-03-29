use std::collections::HashMap;

pub fn text_to_morse(text: &str) -> String {
    let char_map = get_char_to_morse_map();
    
    text.to_uppercase()
        .split(' ')  // Dividir por espacios
        .map(|word| {
            word.chars()
                .map(|c| {
                    char_map.get(&c)
                        .map(|morse| morse.to_string())
                        .unwrap_or_else(|| {
                            eprintln!("⚠️  Character '{}' not supported in morse", c);
                            String::new()
                        })
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" / ")
        .trim()
        .to_string()
}

fn get_char_to_morse_map() -> HashMap<char, &'static str> {
    let mut map = HashMap::new();
    
    // Letras
    map.insert('A', ".-");
    map.insert('B', "-...");
    map.insert('C', "-.-.");
    map.insert('D', "-..");
    map.insert('E', ".");
    map.insert('F', "..-.");
    map.insert('G', "--.");
    map.insert('H', "....");
    map.insert('I', "..");
    map.insert('J', ".---");
    map.insert('K', "-.-");
    map.insert('L', ".-..");
    map.insert('M', "--");
    map.insert('N', "-.");
    map.insert('O', "---");
    map.insert('P', ".--.");
    map.insert('Q', "--.-");
    map.insert('R', ".-.");
    map.insert('S', "...");
    map.insert('T', "-");
    map.insert('U', "..-");
    map.insert('V', "...-");
    map.insert('W', ".--");
    map.insert('X', "-..-");
    map.insert('Y', "-.--");
    map.insert('Z', "--..");
    
    // Números
    map.insert('0', "-----");
    map.insert('1', ".----");
    map.insert('2', "..---");
    map.insert('3', "...--");
    map.insert('4', "....-");
    map.insert('5', ".....");
    map.insert('6', "-....");
    map.insert('7', "--...");
    map.insert('8', "---..");
    map.insert('9', "----.");
    
    // Puntuación
    map.insert('.', ".-.-.-");
    map.insert(',', "--..--");
    map.insert('?', "..--..");
    map.insert('\'', ".----.");
    map.insert('!', "-.-.--");
    map.insert('/', "-..-.");
    map.insert('(', "-.-.-.");
    map.insert(')', "-.--.-");
    map.insert('&', ".-...");
    map.insert(':', "---...");
    map.insert(';', "-.-.-.");
    map.insert('=', "-...-");
    map.insert('+', ".-.-.");
    map.insert('-', "-....-");
    map.insert('_', "..--.-");
    map.insert('"', ".-..-.");
    map.insert('$', "...-..-");
    map.insert('@', ".--.-.");
    
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_letter() {
        let result = text_to_morse("A");
        assert_eq!(result, ".-");
    }

    #[test]
    fn test_simple_word() {
        let result = text_to_morse("SOS");
        assert_eq!(result, "... --- ...");
    }

    #[test]
    fn test_word_with_space() {
        let result = text_to_morse("HI");
        assert_eq!(result, ".... ..");
    }

    #[test]
    fn test_sentence_with_spaces() {
        let result = text_to_morse("A B");
        assert_eq!(result, ".- / -...");
    }

    #[test]
    fn test_number() {
        let result = text_to_morse("123");
        assert_eq!(result, ".---- ..--- ...--");
    }

    #[test]
    fn test_lowercase_conversion() {
        let result = text_to_morse("hello");
        assert_eq!(result, ".... . .-.. .-.. ---");
    }
}
