extern crate clap;

use clap::{Arg, App};

const PROGRAM_DESC: &'static str = "Obtén el checksum de un fichero";
const PROGRAM_NAME: &'static str = "file-checksum";

fn main() {
    let matches = App::new(PROGRAM_NAME)
                        .version("1.0.0")
                        .author("Jesus Gallego")
                        .about(PROGRAM_DESC)
                        .arg(Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .takes_value(true)
                                .required(true))
                        .get_matches();
    
    let file = matches.value_of("file").unwrap();
    let checksum = get_checksum_for_file(&file);
    println!("{}", checksum);
}

fn get_checksum_for_file(path: &str) -> String {
    let content = std::fs::read_to_string(&path).unwrap();
    let computed_checksum = compute_checksum(&content);
    
    format_checksum(computed_checksum)
}

fn compute_checksum(script: &str) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(script);
    hasher.finalize().into()
}

/// The length (in bytes, or equivalently ascii characters) of the checksum
/// strings.
const CHECKSUM_STR_LEN: usize = 64;

fn format_checksum(checksum: [u8; 32]) -> String {
    use std::fmt::Write as _;

    let mut checksum_string = String::with_capacity(32 * 2);

    for byte in checksum {
        write!(checksum_string, "{byte:02x}").unwrap();
    }

    assert_eq!(checksum_string.len(), CHECKSUM_STR_LEN);

    checksum_string
}