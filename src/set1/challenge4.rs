use super::super::utils::{file_helpers, single_byte_xor, xor_score};
use std::time::Duration;

pub fn detect_single_character_xor(duration: &mut Duration) -> xor_score::XorScore {
    let hex_strings: Vec<String> = file_helpers::filename_to_str_vec("src/set1/data/4.txt");

    let mut best = xor_score::XorScore::default();
    for str in hex_strings {
        let this_best = single_byte_xor::decrypt_from_str(&str, duration);

        if this_best.score > best.score {
            best = this_best;
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let best = detect_single_character_xor(&mut Duration::new(0, 0));
        assert_eq!(best.decoding_char(), '5');
        assert_eq!(
            best.decoded_string,
            String::from("Now that the party is jumping")
        );
    }
}
