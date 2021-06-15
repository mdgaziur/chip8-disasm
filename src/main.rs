mod disasm;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Chip 8 Disassembler")
        .version("0.1.0")
        .author("MD Gaziur Rahman Noor <mdgaziurrahmannoor@gmail.com>")
        .about("Makes chip8 bytes readable")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .about("The file to make readable")
            .required(true)
            .takes_value(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .about("The file where disassembled text will be stored")
            .required(true)
            .takes_value(true))
        .get_matches();

    let input = matches.value_of("file").expect("Failed to parse argument");
    let output = matches.value_of("output").expect("Failed to parse argument");

    let bytes = match std::fs::read(input) {
        Ok(b) => b,
        Err(e) => {
            println!("Error occured while trying to open file: {}", e);
            std::process::exit(1)
        }
    };

    let mut disassembler = disasm::init::init_disasm(bytes, output);
    disassembler.disasm();
}