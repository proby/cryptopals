use super::super::utils::hamming;
use std::fs;

pub fn break_repeating_key_xor() {
    let contents =
        fs::read_to_string("src/set1/data/6.txt").expect("Something went wrong reading the file");
    let contents = contents.replace("\n", "");
    // println!("{:?}", contents);

    let contents = base64::decode(contents).expect("Can't decode base64");
    println!(
        "{:?} {}",
        contents.len(),
        hamming::calc_distance("asdf", "sdde")
    );
}
