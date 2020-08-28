mod set1;
mod utils;

fn main() {
    let (best, _best_string) = set1::challenge4::detect_single_character_xor();
    println!("CHALLENGE 4: {}", best.print_info());

    let best = set1::challenge3::single_byte_xor_decrypt(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );
    println!("CHALLENGE 3: {}", best.print_info());

    let result = set1::challenge2::fixed_xor(
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965",
    );
    println!("CHALLENGE 2: {:?}", result);

    let result = set1::challenge1::hex_str_to_base_64_str("1c0111001f010100061a024b53535009181c");
    println!("CHALLENGE 1: {:?}", result);
}
