use super::super::utils::{single_byte_xor, xor_score};
use std::time::Duration;

pub fn single_byte_xor_decrypt(hex_str: &str, duration: &mut Duration) -> xor_score::XorScore {
    single_byte_xor::decrypt_from_str(hex_str, duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let best = single_byte_xor::decrypt_from_str(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
            &mut Duration::new(0, 0),
        );
        assert_eq!(best.decoding_char(), 'X');
        assert_eq!(
            best.decoded_string,
            String::from("Cooking MC's like a pound of bacon")
        );
    }
}
