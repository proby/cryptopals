use std::collections::HashMap;

pub fn score_for(xored_bytes: Vec<u8>) -> f32 {
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

fn chi_squared(histo_b: HashMap<u8, f32>) -> f32 {
    ENGLISH_HISTOGRAM.iter().fold(0.0, |score, (key, val_eng)| {
        let val_b = histo_b.get(key).unwrap_or(&0.0);

        score + (val_eng - val_b).powi(2) / val_eng
    })
}
