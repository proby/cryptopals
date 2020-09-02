mod set1;
mod utils;

use std::time::Instant;

fn run_challenge(challenge_num: usize, with_timing_info: bool) {
    let instant = Instant::now();
    let results_to_print: String;
    match challenge_num {
        1 => {
            results_to_print =
                set1::challenge1::hex_str_to_base_64_str("1c0111001f010100061a024b53535009181c");
        }
        2 => {
            results_to_print = set1::challenge2::fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965",
            );
        }
        3 => {
            let best = set1::challenge3::single_byte_xor_decrypt(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
            );
            results_to_print = best.print_info();
        }
        4 => {
            let (best, _best_string) = set1::challenge4::detect_single_character_xor();
            results_to_print = best.print_info();
        }
        5 => {
            let input =
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
            results_to_print = set1::challenge5::repeating_key_xor(input, "ICE");
        }
        6 => {
            let (key, message) = set1::challenge6::break_repeating_key_xor();
            results_to_print = format!(
                "CHALLENGE 6: key: \"{}\", decrypted len: {}",
                key,
                message.len()
            );
        }
        7 => {
            let message = set1::challenge7::aes_in_ecb_mode();
            results_to_print = message[0..=32].to_string();
        }
        8 => {
            results_to_print = set1::challenge8::detect_aes_in_ecb_mode();
        }
        _ => panic!("challenge number {} is not implemented", challenge_num),
    }

    if with_timing_info {
        let after = instant.elapsed();
        println!(
            "CHALLEGE {} ({:?}): {}",
            challenge_num, after, results_to_print
        );
    } else {
        println!("CHALLEGE {}: {}", challenge_num, results_to_print);
    }
}

fn main() {
    let show_timings = true;
    for challenge_num in 1..=8 {
        run_challenge(challenge_num, show_timings);
    }
}
