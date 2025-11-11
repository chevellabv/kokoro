use log::warn;
use std::{collections::HashMap, sync::LazyLock};

/// Load v0.19 vocabulary from tokens.txt
/// Format: "character token_id" per line
fn load_vocab_from_tokens(tokens_txt: &str) -> HashMap<char, u8> {
    let mut map = HashMap::new();

    for line in tokens_txt.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            // First part is the character (may be multi-byte UTF-8)
            // Second part is the token ID
            if let Ok(token_id) = parts[1].parse::<u8>() {
                // Get the first character (handles multi-byte properly)
                if let Some(ch) = parts[0].chars().next() {
                    map.insert(ch, token_id);
                }
            }
        }
    }

    map
}

// Embed the tokens.txt file at compile time
const TOKENS_V019: &str = include_str!("../dict/v0_19/tokens.txt");

static VOCAB_V019: LazyLock<HashMap<char, u8>> = LazyLock::new(|| {
    let vocab = load_vocab_from_tokens(TOKENS_V019);
    eprintln!("Loaded v0.19 vocabulary: {} tokens", vocab.len());
    vocab
});

/// Convert phoneme string to token IDs for v0.19
pub fn get_token_ids_v019(phonemes: &str) -> Vec<i64> {
    let mut tokens = Vec::with_capacity(phonemes.len() + 2);
    tokens.push(0); // BOS (beginning of sequence) token

    for ch in phonemes.chars() {
        match VOCAB_V019.get(&ch) {
            Some(&token_id) => {
                tokens.push(token_id as i64);
            }
            None => {
                // Only warn for non-space characters
                if ch != ' ' && ch != '\n' && ch != '\t' {
                    warn!("[v0.19] Unknown phone '{}' (U+{:04X}), skipped.", ch, ch as u32);
                }
            }
        }
    }

    tokens.push(0); // EOS (end of sequence) token
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocab_loading() {
        let vocab = &*VOCAB_V019;
        assert!(vocab.len() > 150, "Should have 170+ tokens, got {}", vocab.len());

        // Check some known tokens from tokens.txt
        assert_eq!(vocab.get(&'$'), Some(&0));   // $ 0
        assert_eq!(vocab.get(&';'), Some(&1));   // ; 1
        assert_eq!(vocab.get(&'ɝ'), Some(&88));  // ɝ 88 - This was missing in v1.0!
        assert_eq!(vocab.get(&'ɚ'), Some(&85));  // ɚ 85
        assert_eq!(vocab.get(&'ˈ'), Some(&156)); // ˈ 156 - Primary stress
        assert_eq!(vocab.get(&'a'), Some(&43));  // a 43
        assert_eq!(vocab.get(&'z'), Some(&68));  // z 68
    }

    #[test]
    fn test_tokenization() {
        let tokens = get_token_ids_v019("hello");
        assert_eq!(tokens[0], 0); // BOS
        assert_eq!(tokens[tokens.len()-1], 0); // EOS
        assert!(tokens.len() > 2); // Should have content
    }
}
