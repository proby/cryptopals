pub fn encode(bytes: &[u8]) -> String {
    let capacity = bytes.len() * 2;

    bytes
        .iter()
        .fold(String::with_capacity(capacity), |mut acc, byte| {
            let (upper, lower) = byte_to_hexes(*byte);
            acc.push(upper);
            acc.push(lower);
            acc
        })
}

pub fn decode(hex_str: &str) -> Vec<u8> {
    assert!(hex_str.len() % 2 == 0, "String must be even length");
    let capacity = hex_str.len() / 2;

    let mut prev_char: u8 = 0;
    hex_str
        .bytes()
        .enumerate()
        .fold(Vec::with_capacity(capacity), |mut acc, (index, byte)| {
            if index % 2 == 0 {
                prev_char = raw_byte_to_char_byte(byte);
            } else {
                let two_hex: u8 = prev_char * 16 + raw_byte_to_char_byte(byte);
                acc.push(two_hex);
            }
            acc
        })
}

fn raw_byte_to_char_byte(raw_byte: u8) -> u8 {
    match raw_byte {
        b'0'..=b'9' => raw_byte - b'0',
        b'a'..=b'f' => raw_byte - b'a' + 10,
        b'A'..=b'F' => raw_byte - b'A' + 10,
        _ => panic!(format!("{} is not a valid hex char", raw_byte)),
    }
}

fn byte_to_char_byte(byte: u8) -> u8 {
    match byte {
        0..=9 => b'0' + byte,
        10..=15 => b'a' + (byte - 10),
        _ => panic!(format!("{} is not a valid char", byte)),
    }
}

fn byte_to_hexes(byte: u8) -> (char, char) {
    let lower = byte % 16;
    let upper = (byte - lower) / 16;
    let lower_char = byte_to_char_byte(lower);
    let upper_char = byte_to_char_byte(upper);

    (upper_char as char, lower_char as char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let the_string = "deadbeef";
        assert_eq!(encode(&decode(the_string)), the_string);

        let the_string = "deadbeef01234567ABCDEF";
        assert_eq!(encode(&decode(the_string)), the_string.to_lowercase());
    }
}
