use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};

pub struct CodeGenerator {
    length: usize,
}

impl CodeGenerator {
    pub fn new(length: usize) -> CodeGenerator {
        CodeGenerator { length }
    }

    fn random_bytes_from_alphabet(&self, rng: &mut ThreadRng, alphabet: &[u8]) -> Vec<u8> {
        assert!(!alphabet.is_empty(), "alphabet must not be empty");

        let mut out = Vec::with_capacity(self.length);
        for _ in 0..self.length {
            let idx = rng.random_range(0..alphabet.len());
            out.push(alphabet[idx]);
        }
        out
    }

    pub fn generate(&self) -> Result<String, String> {
        const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxzy";
        let result = self.random_bytes_from_alphabet(&mut thread_rng(), ALPHABET);
        String::from_utf8(result).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_returns_string_with_requested_length() {
        let generator = CodeGenerator::new(8);

        let code = generator.generate().expect("generator should succeed");

        assert_eq!(code.len(), 8);
        assert!(code.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn random_bytes_from_alphabet_respects_supplied_alphabet() {
        let mut rng = thread_rng();
        let generator = CodeGenerator::new(4);

        let bytes = generator.random_bytes_from_alphabet(&mut rng, b"a");

        assert_eq!(bytes, vec![b'a'; 4]);
    }

    #[test]
    #[should_panic(expected = "alphabet must not be empty")]
    fn random_bytes_from_alphabet_panics_on_empty_alphabet() {
        let mut rng = thread_rng();
        let generator = CodeGenerator::new(4);

        let _ = generator.random_bytes_from_alphabet(&mut rng, b"");
    }
}
