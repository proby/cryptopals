#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod set1;
mod set2;
mod utils;

use std::time::{Duration, Instant};

use utils::file_helpers;

fn run_challenge(challenge_num: usize, with_timing_info: bool, total_duration: &mut Duration) {
    let instant = Instant::now();
    let results_to_print: String = match challenge_num {
        1 => set1::challenge1::hex_str_to_base_64_str("1c0111001f010100061a024b53535009181c"),
        2 => set1::challenge2::fixed_xor(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965",
        ),
        3 => {
            let best = set1::challenge3::single_byte_xor_decrypt(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
            );
            best.print_info()
        }
        4 => {
            let hex_strings: Vec<String> = file_helpers::filename_to_str_vec("src/set1/data/4.txt");
            let best = set1::challenge4::detect_single_character_xor(hex_strings);
            best.print_info()
        }
        5 => {
            let input =
                b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
            set1::challenge5::repeating_key_xor(input, b"ICE")
        }
        6 => {
            let contents = file_helpers::filename_to_bytes_vec("src/set1/data/6.txt");
            let (key, message) = set1::challenge6::break_repeating_key_xor(&contents);
            format!(
                "CHALLENGE 6: key: \"{key}\", decrypted len: {}",
                message.len()
            )
        }
        7 => {
            let contents = file_helpers::filename_to_bytes_vec("src/set1/data/7.txt");
            let key = b"YELLOW SUBMARINE";
            let message = set1::challenge7::aec_ecb_decrypt(&contents, key);
            message[0..=32].to_string()
        }
        8 => {
            let hex_strings = file_helpers::filename_to_str_vec("src/set1/data/8.txt");
            set1::challenge8::detect_aes_in_ecb_mode(hex_strings)
        }
        9 => {
            let res = set2::run_challenge_9();
            let str = String::from_utf8(res).unwrap();
            format!("{str:?}")
        }
        10 => {
            let res = set2::run_challenge_10();
            let str = String::from_utf8(res).unwrap();
            str[0..=32].to_string()
        }
        11 => set2::run_challenge_11(),
        12 => {
            let res = set2::run_challenge_12();
            let str = String::from_utf8(res).expect("oops");
            format!("{str:?}")
        }
        _ => panic!("challenge number {} is not implemented", challenge_num),
    };

    if with_timing_info {
        let after = instant.elapsed();
        *total_duration += after;
        println!("CHALLEGE {challenge_num} ({after:?}): {results_to_print}",);
    } else {
        println!("CHALLEGE {challenge_num}: {results_to_print}");
    }
}

fn main() {
    let mut total_duration = Duration::new(0, 0);

    let show_timings = true;
    for challenge_num in 1..=12 {
        run_challenge(challenge_num, show_timings, &mut total_duration);
    }

    println!("\ntotal duration: {total_duration:?}");
}
