use crate::utils::{hex, xor_util};

pub fn repeating_key_xor(input_str: &[u8], key: &[u8]) -> String {
    let xored_bytes = xor_util::xor_byte_vecs(input_str, key);

    hex::encode(&xored_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(repeating_key_xor(input, key), expected_output);
    }
}
