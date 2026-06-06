//! TTS Utility Functions
//! f0rg3d in l0v3 by Ross Edwards & Aurphyx 💎

use std::time::Duration;

/// Calculate audio duration from sample count
pub fn calculate_duration(samples: usize, sample_rate: u32) -> Duration {
    Duration::from_secs_f64(samples as f64 / sample_rate as f64)
}

/// Normalize text for TTS processing
pub fn normalize_text(text: &str) -> String {
    text.trim()
        .replace("\n", " ")
        .replace("\r", "")
        .replace("  ", " ")
}

/// Split text into chunks for streaming
pub fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > max_chars {
            if !current.is_empty() {
                chunks.push(current.trim().to_string());
                current = String::new();
            }
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    
    if !current.is_empty() {
        chunks.push(current.trim().to_string());
    }
    
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_text() {
        let text = "Hello world this is a test of the chunking system";
        let chunks = chunk_text(text, 20);
        assert!(chunks.len() > 1);
        for chunk in &chunks {
            assert!(chunk.len() <= 25); // Allow some overflow for words
        }
    }
    
    #[test]
    fn test_normalize_text() {
        let text = "  Hello\n\nWorld  ";
        assert_eq!(normalize_text(text), "Hello  World");
    }
}
