use hex::FromHex;
use std::fs;
use std::vec::Vec;

pub fn bytevec_from_hexfile(file_path: &String) -> Result<Vec<[u8; 3]>, &'static str> {
    let mut bytevec: Vec<[u8; 3]> = Vec::new();

    for word in fs::read_to_string(file_path).unwrap().split_whitespace() {
        let res = <[u8; 3]>::from_hex(word);

        match res {
            Ok(val) => bytevec.push(val),
            Err(msg) => eprintln!("Warning while parsing hexfile: {}", msg),
        }
    }

    Ok(bytevec)
}
