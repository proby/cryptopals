use crate::utils::{hamming, scorer, single_byte_xor, xor_util};

pub fn break_repeating_key_xor(contents: &[u8]) -> (String, String) {
    let key_sizes_to_test = find_key_sizes_to_test(&contents);
    let possible_keys: Vec<String> = find_possible_keys(&contents, &key_sizes_to_test);

    find_best_score(&contents, &possible_keys)
}

fn calc_sum_of_distances(contents: &[u8], key_size: usize, num_of_blocks_to_assess: usize) -> f32 {
    let mut prev_keysized_block: &[u8] = &[];

    (0..num_of_blocks_to_assess)
        .enumerate()
        .fold(0.0, |mut acc, (index, block_num)| {
            let start_index = key_size * block_num;
            let end_index = key_size * (block_num + 1);
            let keysized_block = contents
                .get(start_index..end_index)
                .expect("Failed to fetch first block");
            if index > 0 {
                let dist = hamming::calc_distance(prev_keysized_block, keysized_block) as f32;
                acc += dist;
            }
            prev_keysized_block = keysized_block;

            acc
        })
}

fn calc_mean_distance(contents: &[u8], key_size: usize) -> f32 {
    let num_of_blocks_to_assess = 4;
    let distances_count = num_of_blocks_to_assess - 1;

    calc_sum_of_distances(&contents, key_size, num_of_blocks_to_assess)
        / ((distances_count * key_size) as f32)
}

fn find_key_sizes_to_test(contents: &[u8]) -> Vec<usize> {
    let key_range_start = 2;
    let key_range_end = 40;
    let num_of_keys_to_test = 3;

    let mut key_sizes: Vec<(usize, f32)> = (key_range_start..=key_range_end)
        .map(|key_size| (key_size, calc_mean_distance(&contents, key_size)))
        .collect();

    key_sizes.sort_unstable_by(|(_a_key, a_score), (_b_key, b_score)| {
        a_score.partial_cmp(b_score).expect("unable to compare")
    });

    key_sizes[0..num_of_keys_to_test]
        .iter()
        .map(|(key, _score)| *key)
        .collect::<Vec<usize>>()
}

fn find_possible_keys(contents: &[u8], key_sizes_to_test: &[usize]) -> Vec<String> {
    key_sizes_to_test
        .iter()
        .map(|&test_key_size| {
            let inner_size = (contents.len() as f32 / test_key_size as f32).ceil() as usize;
            let mut transposed: Vec<Vec<u8>> = vec![Vec::with_capacity(inner_size); test_key_size];
            for (index, byte) in contents.iter().enumerate() {
                let inner_vec = transposed.get_mut(index % test_key_size).expect("NOPE");
                inner_vec.push(*byte);
            }

            transposed
                .iter()
                .fold(String::with_capacity(test_key_size), |mut acc, block| {
                    let this_best = single_byte_xor::decrypt(block);
                    acc.push(this_best.decoding_char());
                    acc
                })
        })
        .collect::<Vec<String>>()
}

fn find_best_score(contents: &[u8], possible_keys: &[String]) -> (String, String) {
    let mut best_score = 0.0;
    let mut best_key = String::new();
    let mut best_str = String::new();

    for possible_key in possible_keys {
        let possible_key_bytes = possible_key.as_bytes();
        let xored = xor_util::xor_byte_vecs(&possible_key_bytes, &contents);
        let score = scorer::score_for(&xored);

        if score > best_score {
            best_score = score;
            best_key = possible_key.to_owned();
            let strstr = String::from_utf8(xored).unwrap();
            best_str = strstr;
        }
    }

    (best_key, best_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file_helpers;

    #[test]
    fn the_example() {
        let contents = file_helpers::filename_to_bytes_vec("src/set1/data/6.txt");
        let (key, message) = break_repeating_key_xor(&contents);
        assert_eq!(key, "Terminator X: Bring the noise");
        assert_eq!(message, "I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n");
    }
}
