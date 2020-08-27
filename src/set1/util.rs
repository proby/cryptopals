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
