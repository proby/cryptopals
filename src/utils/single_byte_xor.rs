use super::{hex, scorer, xor_score, xor_util};

pub fn decrypt_from_str(hex_str: &str) -> xor_score::XorScore {
    let hex_str_as_bytes: Vec<u8> = hex::decode(hex_str);

    decrypt(&hex_str_as_bytes)
}

pub fn decrypt(hex_str_as_bytes: &[u8]) -> xor_score::XorScore {
    let mut best = xor_score::XorScore::default();
    let mut other_vec: Vec<u8>;
    let mut xored_bytes: Vec<u8>;

    for byte_to_test in 0..255 {
        other_vec = vec![byte_to_test; hex_str_as_bytes.len()];
        xored_bytes = xor_util::xor_byte_vecs(hex_str_as_bytes, &other_vec);

        if let Ok(utf_str) = String::from_utf8(xored_bytes.to_owned()) {
            let score = scorer::score_for(&xored_bytes);

            if score > best.score {
                best = xor_score::XorScore {
                    decoding_byte: byte_to_test,
                    score,
                    xored_bytes,
                    decoded_string: utf_str.trim().to_string(),
                }
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_ruby() {
        let score = decrypt(&[28, 30, 10, 3, 78, 79, 27, 100, 72, 15, 18, 10, 79, 0, 113]);
        assert_eq!(score.score, 0.33570558);
        assert_eq!(score.decoding_char(), ' ');
        assert_eq!(score.decoded_string, String::from("<>*#no;Dh/2*o Q"));
    }
}
