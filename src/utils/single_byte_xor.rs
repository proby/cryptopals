use super::{hex, scorer, util, xor_score};

pub fn decrypt_from_str(hex_str: &str) -> xor_score::XorScore {
    let hex_str_as_bytes: Vec<u8> = hex::decode(hex_str);

    decrypt(hex_str_as_bytes)
}

pub fn decrypt(hex_str_as_bytes: Vec<u8>) -> xor_score::XorScore {
    let mut best = xor_score::XorScore::default();
    for byte_to_test in 0..255 {
        let other_vec: Vec<u8> = vec![byte_to_test; hex_str_as_bytes.len()];
        let xored_bytes = util::xor_byte_vecs(&hex_str_as_bytes, &other_vec);

        if let Ok(utf_str) = String::from_utf8(xored_bytes.to_owned()) {
            let score = scorer::score_for(&xored_bytes);
            if score > best.score {
                best.decoding_byte = byte_to_test;
                best.score = score;
                best.xored_bytes = xored_bytes;
                best.decoded_string = utf_str.trim().to_string();
            }
        }
    }

    best
}
