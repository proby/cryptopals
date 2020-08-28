#[derive(Default)]
pub struct XorScore {
    pub decoding_byte: u8,
    pub score: f32,
    pub xored_bytes: Vec<u8>,
    pub decoded_string: String,
}

impl XorScore {
    pub fn decoding_char(&self) -> char {
        self.decoding_byte as char
    }

    pub fn print_info(&self) -> String {
        format!(
            "Best char: {:?} ({:?}) w/ score {} decodes to \"{:}\"",
            self.decoding_char(),
            self.decoding_byte,
            self.score,
            self.decoded_string
        )
    }
}
