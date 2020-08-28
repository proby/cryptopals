use super::util;

fn ones_count(num: u8) -> u32 {
    let mut mut_num = num;
    let mut count = 0;

    while mut_num > 0 {
        if mut_num & 1 == 1 {
            count += 1;
        }
        mut_num /= 2;
    }

    count
}

// The Hamming distance is just the number of differing bits.
pub fn calc_distance(str_a: &str, str_b: &str) -> u32 {
    // convert to bytes
    let bytes_a = str_a.as_bytes().to_vec();
    let bytes_b = str_b.as_bytes().to_vec();

    // xor 'em
    let xored_bytes = util::xor_byte_vecs(bytes_a, bytes_b);

    // count number of bits in result
    xored_bytes.iter().fold(0, |mut acc, byte| {
        acc += ones_count(*byte);
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ones_count_simple() {
        assert_eq!(ones_count(3), 2);
        assert_eq!(ones_count(7), 3);
        assert_eq!(ones_count(2), 1);
        assert_eq!(ones_count(24), 2);
        assert_eq!(ones_count(65), 2);
        assert_eq!(ones_count(73), 3);
        assert_eq!(ones_count(4), 1);
        assert_eq!(ones_count(79), 5);
        assert_eq!(ones_count(10), 2);
        assert_eq!(ones_count(75), 4);
        assert_eq!(ones_count(21), 3);
        assert_eq!(ones_count(68), 2);
        assert_eq!(ones_count(82), 3);
        assert_eq!(ones_count(85), 4);
    }

    #[test]
    fn the_examle() {
        assert_eq!(calc_distance("this is a test", "wokka wokka!!!"), 37);
    }
}
