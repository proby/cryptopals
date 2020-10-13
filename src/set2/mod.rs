use super::utils::{aes_cbc, aes_ecb, encryption_oracle, file_helpers, pkcs_padding};

pub fn run_challenge_9() -> Vec<u8> {
    pkcs_padding::pad(b"YELLOW SUBMARINE", 20)
}

pub fn run_challenge_10() -> Vec<u8> {
    let ciphertext = file_helpers::filename_to_bytes_vec("src/set2/data/10.txt");
    let key = b"YELLOW SUBMARINE";

    aes_cbc::decrypt_with_zero_iv(&ciphertext, key)
}

pub fn run_challenge_11() -> String {
    for _iteration in 1..=20 {
        let input: Vec<u8> = vec![b'A'; 500];
        let (ciphertext, actual_mode) = encryption_oracle::generate_cipher(&input);
        let guessed_mode = encryption_oracle::detect_mode(&ciphertext);

        if actual_mode != guessed_mode {
            return format!(
                "NOT OK! guessed: {} vs actual: {}",
                guessed_mode, actual_mode
            );
        }
    }

    String::from("ALL OK")
}

fn challenge_12_oracle(your_string: &[u8]) -> Vec<u8> {
    let magic_key = [
        53, 178, 195, 6, 132, 136, 97, 212, 43, 160, 107, 88, 105, 183, 124, 171,
    ];

    let magic_base64_contents = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let mut magic_contents = base64::decode(magic_base64_contents).unwrap();

    let mut plaintext = your_string.to_owned();
    plaintext.append(&mut magic_contents);

    aes_ecb::encrypt(&plaintext, &magic_key)
}

pub fn run_challenge_12() -> Vec<u8> {
    // Discover the block size of the cipher.
    let mut my_text = vec![b'A'];
    let ciphertext = challenge_12_oracle(&my_text);
    let ciphertext_len = ciphertext.len();
    let block_size: usize;

    loop {
        my_text.push(b'A');
        let ciphertext = challenge_12_oracle(&my_text);
        if ciphertext.len() > ciphertext_len {
            block_size = ciphertext.len() - ciphertext_len;
            // println!("block size is: {:?} bytes", block_size);
            break;
        }
    }

    // Detect that the function is using ECB.
    let ciphertext = challenge_12_oracle(b"asdfasdfasdfasdfasdfasdfasdfasdfasdf");
    let mode = encryption_oracle::detect_mode(&ciphertext);
    // println!("function is using: {}", mode);

    determine_whole_block(block_size, &mode)
}

fn determine_whole_block(block_size: usize, mode: &str) -> Vec<u8> {
    assert_eq!(mode, "ECB");

    let mut results: Vec<u8> = vec![];
    let mut bytes_to_short = 0;
    let mut prefix_bytes;

    for _ in 0..block_size {
        bytes_to_short += 1;
        prefix_bytes = vec![0; block_size - bytes_to_short];
        let oracle_response = challenge_12_oracle(&prefix_bytes);
        let oracle_response_first_block = oracle_response.get(0..block_size).expect("oops");

        prefix_bytes.append(&mut results.to_owned());
        prefix_bytes.push(0);
        let result = determine_last_byte(
            block_size,
            prefix_bytes.to_owned(),
            oracle_response_first_block,
        )
        .expect("msg");
        results.push(result);
    }

    results
}

fn determine_last_byte(block_size: usize, mut base_bytes: Vec<u8>, target: &[u8]) -> Option<u8> {
    // println!("****");
    // println!("base_bytes ({:?}): {:?}", base_bytes.len(), base_bytes);
    // println!("target ({:?}): {:?}", target.len(), target);

    for last_byte in 0..255 {
        base_bytes[block_size - 1] = last_byte;
        let ciphertext = challenge_12_oracle(&base_bytes);
        let first_block = ciphertext.get(0..(block_size)).expect("oops");
        if first_block == target {
            // println!("found: {:?} ({:?})", last_byte as char, last_byte);
            // println!("****");
            return Some(last_byte);
        }
    }

    // println!("found: NONE");
    // println!("****");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_example_9() {
        assert_eq!(run_challenge_9(), b"YELLOW SUBMARINE\x04\x04\x04\x04");
    }

    #[test]
    fn the_example_10() {
        let expected = b"I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
        assert_eq!(run_challenge_10(), expected);
    }

    #[test]
    fn the_example_11() {
        assert_eq!(run_challenge_11(), "ALL OK".to_string());
    }
}
