use std::collections::HashMap;

use super::hex;

const ENGLISH_HISTOGRAM: [(u8, f32); 25] = [
    (b' ', 0.14),
    (b'e', 0.12),
    (b't', 0.09),
    (b'a', 0.08),
    (b'o', 0.07),
    (b'i', 0.06),
    (b'n', 0.06),
    (b's', 0.06),
    (b'h', 0.06),
    (b'r', 0.05),
    (b'd', 0.04),
    (b'l', 0.04),
    (b'c', 0.02),
    (b'u', 0.02),
    (b'm', 0.02),
    (b'w', 0.02),
    (b'f', 0.02),
    (b'g', 0.02),
    (b'y', 0.01),
    (b'p', 0.01),
    (b'b', 0.01),
    (b'v', 0.01),
    (b'k', 0.01),
    (b'j', 0.01),
    (b'?', 0.09), // the "other" bucket
];

fn english_histogram() -> HashMap<u8, f32> {
    let mut histo: HashMap<u8, f32> = HashMap::with_capacity(ENGLISH_HISTOGRAM.len());
    for (key, val) in ENGLISH_HISTOGRAM.iter() {
        histo.insert(*key, *val);
    }
    histo
}

fn chi_squared(histo_b: HashMap<u8, f32>) -> f32 {
    let mut score: f32 = 0.0;
    let english_hiso = english_histogram();

    for (key, val_eng) in english_hiso.iter() {
        let val_b = histo_b.get(key).unwrap_or(&0.0);

        score += (val_eng - val_b).powi(2) / val_eng;
    }

    score
}

fn score_for(xored_bytes: Vec<u8>) -> f32 {
    let xored_bytes_len: f32 = xored_bytes.len() as f32;
    let mut histogram: HashMap<u8, f32> = HashMap::new();

    for xored_byte in xored_bytes.iter() {
        let mut char_byte = xored_byte.to_ascii_lowercase();
        if !char_byte.is_ascii_alphabetic() && char_byte != b' ' {
            char_byte = b'?';
        }

        let counts = histogram.entry(char_byte).or_insert(0.0);
        *counts += 1.0;
    }

    for (_, val) in histogram.iter_mut() {
        *val /= xored_bytes_len;
    }

    let mut score = 1.0 / chi_squared(histogram.clone());
    if *histogram.get(&b'?').unwrap_or(&0.0) < 0.05 {
        score *= 2.0
    }
    score
}

fn xor_decrypt_and_score(hex_str_as_bytes: Vec<u8>, byte_to_test: u8) -> (f32, Vec<u8>) {
    let hex_str_as_bytes_len = hex_str_as_bytes.len();

    let other_vec: Vec<u8> = vec![byte_to_test; hex_str_as_bytes_len];

    let xored_bytes = hex_str_as_bytes.iter().zip(other_vec.iter()).fold(
        Vec::with_capacity(hex_str_as_bytes_len),
        |mut acc, (a, b)| {
            acc.push(a ^ b);
            acc
        },
    );

    let score = score_for(xored_bytes.clone());

    (score, xored_bytes)
}

pub fn singe_byte_xor_decrypt(hex_str: &str) -> (char, String) {
    let hex_str_as_bytes: Vec<u8> = hex::decode(hex_str);

    let mut bests = (0, 0.0, vec![]);
    for byte_to_test in 0..255 {
        let (score, xored_bytes) = xor_decrypt_and_score(hex_str_as_bytes.clone(), byte_to_test);

        if score > bests.1 {
            bests = (byte_to_test, score, xored_bytes)
        }
    }

    let decoded_string = String::from_utf8(bests.2).unwrap();
    let best_char = bests.0 as char;

    println!(
        "Best char: {:?} ({:?}) w/ score {} decodes to \"{:}\"",
        best_char, bests.0, bests.1, decoded_string
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
