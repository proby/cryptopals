use std::fs;

pub fn xor_byte_vecs(bytes_a: Vec<u8>, bytes_b: Vec<u8>) -> Vec<u8> {
    assert!(bytes_a.len() == bytes_b.len());

    bytes_a
        .iter()
        .zip(bytes_b.iter())
        .fold(Vec::with_capacity(bytes_a.len()), |mut acc, (a, b)| {
            acc.push(a ^ b);
            acc
        })
}

fn read_in_and_trim(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.trim().to_string()
}

pub fn filename_to_str_vec(filename: &str) -> Vec<String> {
    let trimmed = read_in_and_trim(filename);
    trimmed
        .split('\n')
        .map(|x| String::from(x.trim()))
        .collect()
}
