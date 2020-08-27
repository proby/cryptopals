use super::{hex, scorer, util};

#[derive(Default)]
struct XorScore {
    decoding_byte: u8,
    score: f32,
    xored_bytes: Vec<u8>,
}

fn xor_decrypt_and_score(hex_str_as_bytes: Vec<u8>, byte_to_test: u8) -> (f32, Vec<u8>) {
    let other_vec: Vec<u8> = vec![byte_to_test; hex_str_as_bytes.len()];
    let xored_bytes = util::xor_byte_vecs(hex_str_as_bytes, other_vec);

    let score = scorer::score_for(xored_bytes.clone());

    (score, xored_bytes)
}

pub fn singe_byte_xor_decrypt(hex_str: &str) -> (char, String) {
    let hex_str_as_bytes: Vec<u8> = hex::decode(hex_str);

    let mut best: XorScore = XorScore::default();
    for byte_to_test in 0..255 {
        let (score, xored_bytes) = xor_decrypt_and_score(hex_str_as_bytes.clone(), byte_to_test);

        if score > best.score {
            best.decoding_byte = byte_to_test;
            best.score = score;
            best.xored_bytes = xored_bytes;
        }
    }

    let decoded_string = String::from_utf8(best.xored_bytes).unwrap();
    let best_char = best.decoding_byte as char;

    println!(
        "Best char: {:?} ({:?}) w/ score {} decodes to \"{:}\"",
        best_char, best.decoding_byte, best.score, decoded_string
    );

    (best_char, decoded_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        assert_eq!(
            singe_byte_xor_decrypt(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            ),
            ('X', String::from("Cooking MC's like a pound of bacon"))
        );
    }
}
