use std::fs;

pub fn filename_to_str_vec(filename: &str) -> Vec<String> {
    let contents = read_in_filename(filename);

    contents
        .split('\n')
        .map(|x| String::from(x.trim()))
        .collect::<Vec<String>>()
}

pub fn filename_to_bytes_vec(filename: &str) -> Vec<u8> {
    let contents = read_in_filename(filename);

    base64::decode(contents.replace("\n", "")).expect("Can't base64 decode")
}

fn read_in_filename(filename: &str) -> String {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| format!("Something went wrong reading {}", filename))
}
