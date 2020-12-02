use super::aes_ecb;
use std::collections::HashMap;

const KEY: &[u8; 16] = b"0123456789abcdef";

fn encrypt(plaintext: &[u8]) -> Vec<u8> {
    aes_ecb::encrypt(plaintext, KEY)
}

fn decrypt(ciphertext: &[u8]) -> Vec<u8> {
    aes_ecb::decrypt(ciphertext, KEY)
}

fn profile_for(email: &str) -> String {
    let cleared_email = email.replace("&", "");
    let cleared_email = cleared_email.replace("=", "");

    format!("email={}&uid=10&role=user", cleared_email)
}

fn parse(kv_string: &str) -> HashMap<&str, &str> {
    kv_string
        .split('&')
        .fold(HashMap::new(), |mut hash, kv_part| {
            let kv: Vec<&str> = kv_part.split('=').collect();
            assert!(kv.len() == 2);

            hash.insert(kv[0], kv[1]);
            hash
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaa() {
        let plaintext = b"aasljfkladjf asdkfjadflkajdslkfadlfjlkdsjf";

        let ciphertext = encrypt(plaintext);
        let results = decrypt(&ciphertext);

        assert_eq!(plaintext.to_vec(), results);
    }

    #[test]
    fn basic_parse_example() {
        let parsed = parse("foo=bar&baz=qux&zap=zazzle");

        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed.get("foo"), Some(&"bar"));
        assert_eq!(parsed.get("baz"), Some(&"qux"));
        assert_eq!(parsed.get("zap"), Some(&"zazzle"));
    }

    #[test]
    fn basic_profile_for() {
        assert_eq!(
            profile_for("foo@bar.com"),
            "email=foo@bar.com&uid=10&role=user".to_string()
        );
        assert_eq!(
            profile_for("fo==&&o@b&&==ar.c=om"),
            "email=foo@bar.com&uid=10&role=user".to_string()
        );
    }

    #[test]
    fn edge_profile_for() {
        assert_eq!(
            profile_for("foo@bar.com&role=admin").contains("role=admin"),
            false
        );
    }
}
