pub fn pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let padding = block_size - (bytes.len() % block_size);
    let mut padding_bytes = vec![padding as u8; padding];
    let mut byte_vec = bytes.to_vec();
    byte_vec.append(&mut padding_bytes);
    byte_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_zero_pads() {
        assert_eq!(
            pad(b"YELLOW SUBMARINE", 20),
            b"YELLOW SUBMARINE\x04\x04\x04\x04"
        );
        assert_eq!(pad(b"YELLOW SUBMARINE!!!", 20), b"YELLOW SUBMARINE!!!\x01");
    }

    #[test]
    fn zero_pads() {
        assert_eq!(
            pad(b"YELLOW SUBMARINE", 8),
            b"YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08"
        )
    }
}
