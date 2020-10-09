pub fn pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let padding = block_size - (bytes.len() % block_size);
    let mut padding_bytes = vec![padding as u8; padding];
    let mut byte_vec = bytes.to_vec();
    byte_vec.append(&mut padding_bytes);
    byte_vec
}

pub fn strip_padding(output_bytes: &mut Vec<u8>) {
    let len = output_bytes.len();
    let last_byte = output_bytes[len - 1];
    let final_len = len - (last_byte as usize);
    output_bytes.truncate(final_len);
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
            b"blueBOATSandGreen submarine&YELLOW SUBMARINESANDBLACK666\x04\x04\x04\x04"
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
            b"Green submarine&YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08"
        );
        assert_eq!(pad(b"Green submarine&YELLOW SUBMARINE", 16), b"Green submarine&YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10");
    }

    #[test]
    fn strip_pads() {
        let mut bytes = b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec();
        strip_padding(&mut bytes);
        assert_eq!(bytes, b"YELLOW SUBMARINE");

        let mut bytes = b"YELLOW SUBMARINE!!!\x01".to_vec();
        strip_padding(&mut bytes);
        assert_eq!(bytes, b"YELLOW SUBMARINE!!!");

        let mut bytes =
            b"blueBOATSandGreen submarine&YELLOW SUBMARINESANDBLACK666\x04\x04\x04\x04".to_vec();
        strip_padding(&mut bytes);
        assert_eq!(
            bytes,
            b"blueBOATSandGreen submarine&YELLOW SUBMARINESANDBLACK666"
        );
    }

    #[test]
    fn strip_zero_pads() {
        let mut bytes = b"YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".to_vec();
        strip_padding(&mut bytes);
        assert_eq!(bytes, b"YELLOW SUBMARINE");

        let mut bytes =
            b"YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10"
                .to_vec();
        strip_padding(&mut bytes);
        assert_eq!(bytes, b"YELLOW SUBMARINE");

        let mut bytes =
            b"Green submarine&YELLOW SUBMARINE\x08\x08\x08\x08\x08\x08\x08\x08".to_vec();
        strip_padding(&mut bytes);
        assert_eq!(bytes, b"Green submarine&YELLOW SUBMARINE");

        let mut bytes = b"Green submarine&YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10".to_vec();
        strip_padding(&mut bytes);
        assert_eq!(bytes, b"Green submarine&YELLOW SUBMARINE");
    }
}
