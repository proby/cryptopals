use super::utils::pkcs_padding;

pub fn run_challenge_9() -> Vec<u8> {
    pkcs_padding::pad(b"YELLOW SUBMARINE", 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example_9() {
        assert_eq!(run_challenge_9(), b"YELLOW SUBMARINE\x04\x04\x04\x04");
    }
}
