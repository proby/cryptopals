use regex::bytes::Regex;

pub fn is_displayable_bytes(bytes: &[u8]) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\A([[:print:]]|\n)*\z").unwrap();
    }
    RE.is_match(bytes)
}
