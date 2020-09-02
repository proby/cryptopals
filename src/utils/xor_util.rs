use std::cmp::Ordering;

pub fn xor_byte_vecs(bytes_a: &[u8], bytes_b: &[u8]) -> Vec<u8> {
    match bytes_a.len().cmp(&bytes_b.len()) {
        Ordering::Equal => xor_byte_vecs_even(bytes_a, bytes_b),
        Ordering::Greater => xor_byte_vecs_uneven(bytes_a, bytes_b),
        Ordering::Less => xor_byte_vecs_uneven(bytes_b, bytes_a),
    }
}

fn xor_byte_vecs_even(bytes_a: &[u8], bytes_b: &[u8]) -> Vec<u8> {
    bytes_a
        .iter()
        .zip(bytes_b.iter())
        .fold(Vec::with_capacity(bytes_a.len()), |mut acc, (a, b)| {
            acc.push(a ^ b);
            acc
        })
}

fn xor_byte_vecs_uneven(bytes_a: &[u8], bytes_b: &[u8]) -> Vec<u8> {
    let bytes_b_len = bytes_b.len();

    bytes_a.iter().enumerate().fold(
        Vec::with_capacity(bytes_a.len()),
        |mut acc, (index, byte)| {
            let other_byte = bytes_b[index % bytes_b_len];
            acc.push(byte ^ other_byte);
            acc
        },
    )
}
