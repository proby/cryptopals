use std::collections::HashSet;

pub fn detect_mode(ciphertext: &[u8]) -> String {
    let chunk_size = ciphertext.len() / 16;
    let mut seen_chunks = HashSet::new();
    for chunk in ciphertext.chunks(16) {
        seen_chunks.insert(chunk);
    }
    if seen_chunks.len() == chunk_size {
        String::from("CBC")
    } else {
        String::from("ECB")
    }
}
