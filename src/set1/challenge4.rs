use super::super::utils::{single_byte_xor, util, xor_score};

pub fn detect_single_character_xor() -> (xor_score::XorScore, String) {
    let hex_strings: Vec<String> = util::filename_to_str_vec("src/set1/data/4.txt");

    let mut best = xor_score::XorScore::default();
    let mut best_input_string = String::from("");
    for str in hex_strings {
        let this_best = single_byte_xor::decrypt_from_str(&str);

        if this_best.score > best.score {
            best = this_best;
            best_input_string = str;
        }
    }

    (best, best_input_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let (best, best_input_string) = detect_single_character_xor();
        assert_eq!(best.decoding_char(), '5');
        assert_eq!(
            best.decoded_string,
            String::from("Now that the party is jumping")
        );
        assert_eq!(
            best_input_string,
            String::from("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f")
        );
    }
}
