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
        assert_eq!(
            pad(
                b"blueBOATSandGreen submarine&YELLOW SUBMARINESANDBLACK666",
                20
            ),
            "blueBOATSandGreen submarine&YELLOW SUBMARINESANDBLACK666\x04\x04\x04\x04".as_bytes()
        );
    }

    #[test]
    fn zero_pads() {
        assert_eq!(
            pad(b"YELLOW SUBMARINE", 8),
            b"YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08"
        );
        assert_eq!(
            pad(b"YELLOW SUBMARINE", 16),
            b"YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10"
        );
        assert_eq!(
            pad(b"Green submarine&YELLOW SUBMARINE", 8),
            "Green submarine&YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".as_bytes()
        );
        assert_eq!(
            pad(b"Green submarine&YELLOW SUBMARINE", 16),
            "Green submarine&YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10".as_bytes()
        );
    }
}
