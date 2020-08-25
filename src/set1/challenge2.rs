fn hex_str_to_bytes_vec(hex_str: &str) -> Vec<u8> {
    hex::decode(hex_str).expect("Failed to decode")
}

pub fn fixed_xor(hex_str_a: &str, hex_str_b: &str) -> String {
    assert!(hex_str_a.len() == hex_str_b.len());

    let bytes_a = hex_str_to_bytes_vec(hex_str_a);
    let bytes_b = hex_str_to_bytes_vec(hex_str_b);

    let xored_bytes = bytes_a.iter().zip(bytes_b.iter()).fold(
        Vec::with_capacity(hex_str_a.len()),
        |mut acc, (a, b)| {
            acc.push(a ^ b);
            acc
        },
    );

    hex::encode(xored_bytes)
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
