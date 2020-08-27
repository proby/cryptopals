use super::{hex, scorer, util};

fn xor_decrypt_and_score(hex_str_as_bytes: Vec<u8>, byte_to_test: u8) -> (f32, Vec<u8>) {
    let other_vec: Vec<u8> = vec![byte_to_test; hex_str_as_bytes.len()];
    let xored_bytes = util::xor_byte_vecs(hex_str_as_bytes, other_vec);

    let score = scorer::score_for(xored_bytes.clone());

    (score, xored_bytes)
}

pub fn decrypt(hex_str: &str) -> scorer::XorScore {
    let hex_str_as_bytes: Vec<u8> = hex::decode(hex_str);

    let mut best = scorer::XorScore::default();
    for byte_to_test in 0..255 {
        let (score, xored_bytes) = xor_decrypt_and_score(hex_str_as_bytes.clone(), byte_to_test);

        if score > best.score {
            best.decoding_byte = byte_to_test;
            best.score = score;
            best.xored_bytes = xored_bytes;
        }
    }

    let utf8_str = String::from_utf8(best.xored_bytes.clone());
    if utf8_str.is_err() {
        best.score = 0.0;
        best.decoded_string = String::from("NOT UTF_8");
        return best;
    }

    best.decoded_string = utf8_str.unwrap().trim().to_string();

    best
}
