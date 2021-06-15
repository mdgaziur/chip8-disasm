use super::disassembler::Disassembler;

pub fn init_disasm(bytes: Vec<u8>, output: &str) -> Disassembler {
    Disassembler::new(bytes, output)
}