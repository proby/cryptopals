use super::utils::{
    aes_cbc, encryption_oracle, encryption_oracle::ECBOracle, file_helpers, oracle_tools,
    pkcs_padding,
};

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
        let (ciphertext, actual_mode) = encryption_oracle::encrypt_with_random_aes_mode(&input);
        let guessed_mode = oracle_tools::detect_mode(&ciphertext);

        if actual_mode != guessed_mode {
            return format!(
                "NOT OK! guessed: {} vs actual: {}",
                guessed_mode, actual_mode
            );
        }
    }

    String::from("ALL OK")
}

fn find_block_size(oracle: &ECBOracle) -> usize {
    let mut my_text = vec![b'A'];
    let ciphertext = oracle.encrypt(&my_text);
    let ciphertext_len = ciphertext.len();

    loop {
        my_text.push(b'A');
        let ciphertext = oracle.encrypt(&my_text);
        if ciphertext.len() > ciphertext_len {
            return ciphertext.len() - ciphertext_len;
        }
    }
}

pub fn run_challenge_12() -> Vec<u8> {
    let oracle = ECBOracle::new();

    // Discover the block size of the cipher.
    let block_size = find_block_size(&oracle);

    // Detect that the function is using ECB.
    let ciphertext = oracle.encrypt(b"asdfasdfasdfasdfasdfasdfasdfasdfasdf");
    assert_eq!(oracle_tools::detect_mode(&ciphertext), "ECB");

    // Determine total blocks needed
    let ciphertext = oracle.encrypt(b"");
    let blocks_needed = ciphertext.len() / block_size;

    let mut total_result = Vec::with_capacity(ciphertext.len());

    for block_number in 1..=blocks_needed {
        let mut block_result = determine_block(&oracle, block_size, block_number, &total_result);
        total_result.append(&mut block_result);
    }

    total_result
}

fn determine_block(
    oracle: &ECBOracle,
    block_size: usize,
    block_number: usize,
    previous_block_results: &[u8],
) -> Vec<u8> {
    let mut results: Vec<u8> = Vec::with_capacity(block_size);
    let mut prefix_bytes;
    let block_offset = block_size * block_number;

    for bytes_to_short in 1..=block_size {
        prefix_bytes = vec![0; block_size - bytes_to_short];

        let oracle_response = oracle.encrypt(&prefix_bytes);
        let oracle_response_target = oracle_response.get(0..block_offset).expect("oops");

        // TODO try using #remove instead of having to append all of the time
        prefix_bytes.append(&mut previous_block_results.to_owned());
        prefix_bytes.append(&mut results.to_owned());
        prefix_bytes.push(0);

        let result = determine_last_byte(
            &oracle,
            block_offset,
            prefix_bytes.to_owned(),
            oracle_response_target,
        )
        .expect("Failed to determine last byte");

        // TODO improve this check
        if result == 1 {
            break;
        }
        results.push(result);
    }

    results
}

// cheater const to make this go faster
const BYTES_TO_TRY: &[u8; 37] = &[
    32, 111, 105, 110, 121, 116, 115, 97, 108, 10, 104, 114, 100, 109, 103, 119, 98, 101, 117, 112,
    118, 106, 82, 39, 53, 46, 48, 87, 45, 99, 84, 68, 63, 78, 44, 73, 1,
];
fn determine_last_byte(
    oracle: &ECBOracle,
    block_offset: usize,
    mut prefix_bytes: Vec<u8>,
    target: &[u8],
) -> Option<u8> {
    for last_byte in BYTES_TO_TRY {
        // used to be "for last_byte in 0..127"
        prefix_bytes[block_offset - 1] = *last_byte;
        let oracle_response = oracle.encrypt(&prefix_bytes);
        let blocks = oracle_response.get(0..block_offset).expect("oops");
        if blocks == target {
            return Some(*last_byte);
        }
    }

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

    #[test]
    fn the_example_12() {
        let expected = b"Rollin\' in my 5.0\nWith my rag-top down so my hair can blow\nThe girlies on standby waving just to say hi\nDid you stop? No, I just drove by\n";
        assert_eq!(run_challenge_12(), expected);
    }
}
