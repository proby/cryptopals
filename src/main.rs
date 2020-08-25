mod set1;

fn main() {
    set1::challenge3::singe_byte_xor_decrypt(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );

    let result = set1::challenge1::hex_str_to_base_64_str("1c0111001f010100061a024b53535009181c");
    println!("{:?}", result);

    let result = set1::challenge2::fixed_xor(
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965",
    );
    println!("{:?}", result);
}
