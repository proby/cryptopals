use super::xor_util;

pub fn calc_distance(bytes_a: &[u8], bytes_b: &[u8]) -> u32 {
    let xored_bytes = xor_util::xor_byte_vecs(bytes_a, bytes_b);

    xored_bytes.iter().fold(0, |mut sum, byte| {
        sum += ones_count(*byte);
        sum
    })
}

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
        assert_eq!(calc_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }
}
