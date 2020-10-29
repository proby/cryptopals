use crate::utils::{hex, oracle_tools};

pub fn detect_aes_in_ecb_mode(hex_strings: Vec<String>) -> String {
    let mut detected = String::from("none found");

    for hex_string in hex_strings {
        let decoded = hex::decode(&hex_string);

        let result = oracle_tools::detect_mode(&decoded);
        if &result == "ECB" {
            detected = hex_string;
            break;
        }
    }
    detected
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file_helpers;

    #[test]
    fn the_example() {
        let hex_strings = file_helpers::filename_to_str_vec("src/set1/data/8.txt");
        let message = detect_aes_in_ecb_mode(hex_strings);
        assert_eq!(message, "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a");
    }
}
