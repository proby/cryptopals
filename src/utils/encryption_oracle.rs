use super::{aes_cbc, aes_ecb};
use rand::{random, rngs::ThreadRng, thread_rng, Rng, RngCore};

pub struct ECBOracle {
    magic_contents: Vec<u8>,
    magic_key: Vec<u8>,
}

const MAGIC_BASE64_VALUE: &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
impl ECBOracle {
    pub fn new() -> Self {
        ECBOracle {
            magic_contents: base64::decode(MAGIC_BASE64_VALUE).unwrap(),
            magic_key: random_bytes(16, thread_rng()),
        }
    }

    pub fn encrypt(&self, prefix_bytes: &[u8]) -> Vec<u8> {
        let mut plaintext = prefix_bytes.to_owned();
        plaintext.append(&mut self.magic_contents.to_owned());

        aes_ecb::encrypt(&plaintext, &self.magic_key)
    }
}

pub fn encrypt_with_random_aes_mode(my_input: &[u8]) -> (Vec<u8>, String) {
    let rng = thread_rng();

    let random_key = random_bytes(16, rng);
    let contents = build_contents(my_input, rng);

    let ciphertext;
    let actual_mode;
    if random() {
        ciphertext = aes_cbc::encrypt_with_zero_iv(&contents, &random_key);
        actual_mode = String::from("CBC");
    } else {
        ciphertext = aes_ecb::encrypt(&contents, &random_key);
        actual_mode = String::from("ECB");
    }

    (ciphertext, actual_mode)
}

fn build_contents(core_contents: &[u8], rng: ThreadRng) -> Vec<u8> {
    let mut contents = thing(rng);
    let mut core_contents = core_contents.to_vec();
    let mut postfix_data = thing(rng);

    contents.append(&mut core_contents);
    contents.append(&mut postfix_data);

    contents
}

fn thing(mut rng: ThreadRng) -> Vec<u8> {
    let extra_bytes_count = rng.gen_range(5, 11);

    random_bytes(extra_bytes_count, rng)
}

fn random_bytes(num_bytes: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut bytes = vec![0u8; num_bytes];

    rng.fill_bytes(&mut bytes);

    bytes
}
