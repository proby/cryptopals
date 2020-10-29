use crate::utils::hex;

pub fn hex_str_to_base_64_str(hex_str: &str) -> String {
    let str_to_encode = hex::decode(hex_str);
    base64::encode(str_to_encode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        // "I'm killing your brain like a poisonous mushroom"
        assert_eq!(
            hex_str_to_base_64_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn short() {
        // "hello rust folks!"
        assert_eq!(
            hex_str_to_base_64_str("68656c6c6f207275737420666f6c6b7321"),
            "aGVsbG8gcnVzdCBmb2xrcyE="
        );
    }

    #[test]
    #[should_panic]
    fn error_raw_byte_to_char_byte() {
        hex_str_to_base_64_str("zzzz");
    }
}
