use super::{aes_cbc, aes_ecb};
use rand::{random, rngs::ThreadRng, thread_rng, Rng, RngCore};
use std::collections::HashSet;

pub fn detect_mode(ciphertext: &[u8]) -> String {
    let chunk_size = ciphertext.len() / 16;
    let mut seen_chunks = HashSet::new();
    for chunk in ciphertext.chunks(16) {
        seen_chunks.insert(chunk);
    }
    if seen_chunks.len() == chunk_size {
        String::from("CBC")
    } else {
        String::from("ECB")
    }
}

pub fn generate_cipher(my_input: &[u8]) -> (Vec<u8>, String) {
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
