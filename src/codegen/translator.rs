use crate::ir::IR;

pub fn generate(ir: IR) -> String {
    ir.letters.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_generation() {
        let ir = IR {
            letters: vec!['S', 'O', 'S'],
        };
        assert_eq!(generate(ir), "SOS");
    }
}
