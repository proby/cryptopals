use super::super::utils::{hex, util};

pub fn fixed_xor(hex_str_a: &str, hex_str_b: &str) -> String {
    let bytes_a = hex::decode(hex_str_a);
    let bytes_b = hex::decode(hex_str_b);

    let xored_bytes = util::xor_byte_vecs(&bytes_a, &bytes_b);

    hex::encode(&xored_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    #[should_panic]
    fn bad() {
        fixed_xor(
            "1c0111001f010100061a024b53535009181cz",
            "686974207468652062756c6c277320657965z",
        );
    }
}
