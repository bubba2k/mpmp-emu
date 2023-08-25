use hex::FromHex;
use std::fs;
use std::io;
use std::vec::Vec;

pub fn bytevec_from_hexfile(file_path: String) -> io::Result<Vec<[u8; 3]>> {
    let mut bytevec: Vec<[u8; 3]> = Vec::new();

    let text = fs::read_to_string(file_path)?;

    // Split the file into words and attempt to parse them
    for word in text.split_whitespace() {
        let res = <[u8; 3]>::from_hex(String::from("0") + word);

        // If we come across a non-hex word, we simply display a warning
        // and ignore it. Probably should figure out a better solution later.
        match res {
            Ok(val) => bytevec.push(val),
            Err(msg) => (), // eprintln!("Warning while parsing hexfile: {}", msg),
        }
    }

    Ok(bytevec)
}
