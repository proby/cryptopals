use super::super::utils::{file_helpers, hex};
use std::collections::HashSet;

pub fn detect_aes_in_ecb_mode() -> String {
    let hex_strings = file_helpers::filename_to_str_vec("src/set1/data/8.txt");
    let mut detected = String::from("none found");

    for hex_string in hex_strings {
        let decoded = hex::decode(&hex_string);
        let chunked: Vec<&[u8]> = decoded.chunks(16).collect();
        let mut seen_chunks = HashSet::new();
        for chunk in chunked {
            seen_chunks.insert(chunk);
        }
        if seen_chunks.len() < 10 {
            detected = hex_string;
            break;
        }
    }
    detected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let message = detect_aes_in_ecb_mode();
        assert_eq!(message, "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a");
    }
}
