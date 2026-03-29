use crate::lexer::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for token in tokens {
        match token {
            Token::Dot => current.push('.'),
            Token::Dash => current.push('-'),
            Token::Space => {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
            }
            _ => {}
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_morse() {
        let tokens = vec![
            Token::Dot,
            Token::Dot,
            Token::Dot,
            Token::Space,
            Token::Dash,
            Token::Dash,
            Token::Dash,
        ];
        let result = parse(tokens);
        assert_eq!(result, vec!["...", "---"]);
    }
}
