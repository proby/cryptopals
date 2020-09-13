use super::super::utils::{single_byte_xor, xor_score};

pub fn single_byte_xor_decrypt(hex_str: &str) -> xor_score::XorScore {
    single_byte_xor::decrypt_from_str(hex_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let best = single_byte_xor::decrypt_from_str(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        );
        assert_eq!(best.decoding_char(), 'X');
        assert_eq!(
            best.decoded_string,
            String::from("Cooking MC's like a pound of bacon")
        );
    }
}
