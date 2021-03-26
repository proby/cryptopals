use super::{aes_cbc, aes_ecb};
use rand::{random, rngs::ThreadRng, thread_rng, Rng, RngCore};

pub struct EcbOracle {
    secret_contents: Vec<u8>,
    key: Vec<u8>,
}

impl EcbOracle {
    pub fn new() -> Self {
        EcbOracle {
            secret_contents: base64::decode(EcbOracle::SECRET_CONTENTS_BASE64).unwrap(),
            key: random_bytes(16, thread_rng()),
        }
    }

    pub fn encrypt(&self, prefix_bytes: &[u8]) -> Vec<u8> {
        let mut plaintext = prefix_bytes.to_owned();
        plaintext.append(&mut self.secret_contents.to_owned());

        aes_ecb::encrypt(&plaintext, &self.key)
    }

    const SECRET_CONTENTS_BASE64: &'static str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
}

pub struct AesModeOracle {
    rng: ThreadRng,
}

impl AesModeOracle {
    pub fn new() -> Self {
        AesModeOracle { rng: thread_rng() }
    }

    pub fn encrypt_with_random_aes_mode(&mut self, my_input: &[u8]) -> (Vec<u8>, String) {
        let random_key = random_bytes(16, self.rng);
        let contents = self.build_contents(my_input);

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

    fn build_contents(&mut self, core_contents: &[u8]) -> Vec<u8> {
        let mut contents = self.generate_random_byte_vec();
        let mut core_contents = core_contents.to_vec();
        let mut postfix_data = self.generate_random_byte_vec();

        contents.append(&mut core_contents);
        contents.append(&mut postfix_data);

        contents
    }

    fn generate_random_byte_vec(&mut self) -> Vec<u8> {
        let extra_bytes_count = self.rng.gen_range(5, 11);

        random_bytes(extra_bytes_count, self.rng)
    }
}

fn random_bytes(num_bytes: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut bytes = vec![0u8; num_bytes];

    rng.fill_bytes(&mut bytes);

    bytes
}
