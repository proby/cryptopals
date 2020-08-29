use super::super::utils::{hex, util};

pub fn repeating_key_xor(input_str: &str, key: &str) -> String {
    let input_byte_vec = input_str.as_bytes();
    let key_vec = key.as_bytes();

    let xored_bytes = util::xor_byte_vecs(&input_byte_vec, &key_vec);

    hex::encode(&xored_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(repeating_key_xor(input, key), expected_output);
    }
}
