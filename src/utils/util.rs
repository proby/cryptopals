use std::fs;

pub fn xor_byte_vecs(bytes_a: Vec<u8>, bytes_b: Vec<u8>) -> Vec<u8> {
    assert!(bytes_a.len() == bytes_b.len());

    // TODO see the perf difference between xor_byte_vecs_uneven and the impl below
    xor_byte_vecs_uneven(bytes_a, bytes_b)

    // bytes_a
    //     .iter()
    //     .zip(bytes_b.iter())
    //     .fold(Vec::with_capacity(bytes_a.len()), |mut acc, (a, b)| {
    //         acc.push(a ^ b);
    //         acc
    //     })
}

pub fn xor_byte_vecs_uneven(bytes_a: Vec<u8>, bytes_b: Vec<u8>) -> Vec<u8> {
    // bytes_a cannot be shorter
    let bytes_b_len = bytes_b.len();
    assert!(bytes_a.len() >= bytes_b_len);

    bytes_a
        .iter()
        .enumerate()
        .fold(Vec::with_capacity(bytes_a.len()), |mut acc, (index, byte)| {
            let other_byte = bytes_b[index % bytes_b_len];
            acc.push(byte ^ other_byte);
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
