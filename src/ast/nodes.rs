#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AstNode {
    Letter(String),
    Word(Vec<AstNode>),
    Sentence(Vec<AstNode>),
}

impl AstNode {
    #[allow(dead_code)]
    pub fn new_letter(value: String) -> Self {
        AstNode::Letter(value)
    }

    #[allow(dead_code)]
    pub fn new_word(nodes: Vec<AstNode>) -> Self {
        AstNode::Word(nodes)
    }

    #[allow(dead_code)]
    pub fn new_sentence(nodes: Vec<AstNode>) -> Self {
        AstNode::Sentence(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_creation() {
        let node = AstNode::new_letter("S".to_string());
        assert_eq!(node, AstNode::Letter("S".to_string()));
    }

    #[test]
    fn test_word_creation() {
        let nodes = vec![
            AstNode::Letter("H".to_string()),
            AstNode::Letter("I".to_string()),
        ];
        let word = AstNode::new_word(nodes);
        match word {
            AstNode::Word(n) => assert_eq!(n.len(), 2),
            _ => panic!("Expected Word"),
        }
    }
}

