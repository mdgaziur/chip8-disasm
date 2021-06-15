pub struct Disassembler {
    bytes: Vec<u8>,
    output: String,
    pc: usize,
    disassembled: String
}

impl Disassembler {
    pub fn new(bytes: Vec<u8>, output: &str) -> Self {
        Disassembler {
            bytes: bytes.clone(),
            output: String::from(output),
            pc: 0,
            disassembled: String::new()
        }
    }

    fn get_opcode(&self) -> u16 {
        (self.bytes[self.pc] as u16) << 8 | (self.bytes[self.pc + 1] as u16)
    }

    pub fn disasm(&mut self) {
        while self.pc < self.bytes.len() {
            let opcode = self.get_opcode();

            let nibbles = (
                (opcode & 0xF000) >> 12 as u8,
                (opcode & 0x0F00) >> 8 as u8,
                (opcode & 0x00F0) >> 4 as u8,
                (opcode & 0x000F) as u8,
            );

            // Super chip-8 ins
            let nnn = (opcode & 0x0FFF) as usize;
            let kk = (opcode & 0x00FF) as u8;
            let x = nibbles.1 as usize;
            let y = nibbles.2 as usize;
            let n = nibbles.3 as usize;

            match nibbles {
                (0x00, 0x00, 0x0e, 0x00) => self.op00e0(),
                (0x00, 0x00, 0x0e, 0x0e) => self.op00ee(),
                (0x01, _, _, _) => self.op1nnn(nnn),
                (0x02, _, _, _) => self.op2nnn(nnn),
                (0x03, _, _, _) => self.op3xkk(x, kk),
                (0x04, _, _, _) => self.op4xkk(x, kk),
                (0x05, _, _, 0x00) => self.op5xy0(x, y),
                (0x06, _, _, _) => self.op6xkk(x, kk),
                (0x07, _, _, _) => self.op7xkk(x, kk),
                (0x08, _, _, 0x00) => self.op8xy0(x, y),
                (0x08, _, _, 0x01) => self.op8xy1(x, y),
                (0x08, _, _, 0x02) => self.op8xy2(x, y),
                (0x08, _, _, 0x03) => self.op8xy3(x, y),
                (0x08, _, _, 0x04) => self.op8xy4(x, y),
                (0x08, _, _, 0x05) => self.op8xy5(x, y),
                (0x08, _, _, 0x06) => self.op8x06(x),
                (0x08, _, _, 0x07) => self.op8xy7(x, y),
                (0x08, _, _, 0x0e) => self.op8x0e(x),
                (0x09, _, _, 0x00) => self.op9xy0(x, y),
                (0x0a, _, _, _) => self.opannn(nnn),
                (0x0b, _, _, _) => self.opbnnn(nnn),
                (0x0c, _, _, _) => self.opcxkk(x, kk),
                (0x0d, _, _, _) => self.opdxyn(x, y, n),
                (0x0e, _, 0x09, 0x0e) => self.opex9e(x),
                (0x0e, _, 0x0a, 0x01) => self.opexa1(x),
                (0x0f, _, 0x00, 0x07) => self.opfx07(x),
                (0x0f, _, 0x00, 0x0a) => self.opfx0a(x),
                (0x0f, _, 0x01, 0x05) => self.opfx15(x),
                (0x0f, _, 0x01, 0x08) => self.opfx18(x),
                (0x0f, _, 0x01, 0x0e) => self.opfx1e(x),
                (0x0f, _, 0x02, 0x09) => self.opfx29(x),
                (0x0f, _, 0x03, 0x03) => self.opfx33(x),
                (0x0f, _, 0x05, 0x05) => self.opfx55(x),
                (0x0f, _, 0x06, 0x05) => self.opfx65(x),
                _ => ()
            }

            self.pc += 2;
        }

        std::fs::write(self.output.clone(), self.disassembled.clone()).expect("Failed to write file!");
    }

    fn op00e0(&mut self) {
        self.disassembled += "CLS\n";
    }

    fn op00ee(&mut self) {
        self.disassembled += "RET\n";
    }

    fn op1nnn(&mut self, nnn: usize) {
        self.disassembled += &format!("JMP {}\n", nnn);
    }

    fn op2nnn(&mut self, nnn: usize) {
        self.disassembled += &format!("CALL {}\n", nnn);
    }

    fn op3xkk(&mut self, x: usize, kk: u8) {
        self.disassembled += &format!("SE V{} {}\n", x, kk);
    }

    fn op4xkk(&mut self, x: usize, kk: u8) {
        self.disassembled += &format!("SNE V{} {}\n", x, kk);
    }

    fn op5xy0(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("SE V{} V{}\n", x, y);
    }

    fn op6xkk(&mut self, x: usize, kk: u8) {
        self.disassembled += &format!("LD V{} {}\n", x, kk);
    }

    fn op7xkk(&mut self, x: usize, kk: u8) {
        self.disassembled += &format!("ADD V{} {}\n", x, kk);
    }

    fn op8xy0(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("LD V{} V{}\n", x, y);
    }

    fn op8xy1(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("OR V{} V{}\n", x, y);
    }

    fn op8xy2(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("AND V{} V{}\n", x, y);
    }

    fn op8xy3(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("XOR V{} V{}\n", x, y);
    }

    fn op8xy4(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("ADD V{} V{}\n", x, y);
    }

    fn op8xy5(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("SUB V{} V{}\n", x, y);
    }

    fn op8x06(&mut self, x: usize) {
        self.disassembled += &format!("SHR V{}\n", x);
    }

    fn op8xy7(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("SUBN V{} V{}\n", x, y);
    }

    fn op8x0e(&mut self, x: usize) {
        self.disassembled += &format!("SHL V{}\n", x);
    }

    fn op9xy0(&mut self, x: usize, y: usize) {
        self.disassembled += &format!("SNE V{} V{}\n", x, y);
    }

    fn opannn(&mut self, nnn: usize) {
        self.disassembled += &format!("LD I {}\n", nnn);
    }

    fn opbnnn(&mut self, nnn: usize) {
        self.disassembled += &format!("JMP V0 {}\n", nnn);
    }

    fn opcxkk(&mut self, x: usize, kk: u8) {
        self.disassembled += &format!("RND V{} {}\n", x, kk);
    }

    fn opdxyn(&mut self, x: usize, y: usize, n: usize) {
        self.disassembled += &format!("DRW V{} V{} {}\n", x, y, n);
    }
    
    fn opex9e(&mut self, x: usize) {
        self.disassembled += &format!("SKP V{}\n", x);
    }

    fn opexa1(&mut self, x: usize) {
        self.disassembled += &format!("SKNP V{}\n", x);
    }

    fn opfx07(&mut self, x: usize) {
        self.disassembled += &format!("LD V{} DT\n", x);
    }

    fn opfx0a(&mut self, x: usize) {
        self.disassembled += &format!("LD V{} K\n", x);
    }

    fn opfx15(&mut self, x: usize) {
        self.disassembled += &format!("LD DT V{}\n", x);
    }

    fn opfx18(&mut self, x: usize) {
        self.disassembled += &format!("LD ST V{}\n", x);
    }

    fn opfx1e(&mut self, x: usize) {
        self.disassembled += &format!("ADD I V{}\n", x);
    }

    fn opfx29(&mut self, x: usize) {
        self.disassembled += &format!("LD F V{}\n", x);
    }

    fn opfx33(&mut self, x: usize) {
        self.disassembled += &format!("LD B V{}\n", x);
    }

    fn opfx55(&mut self, x: usize) {
        self.disassembled += &format!("LD [I] V{}\n", x);
    }

    fn opfx65(&mut self, x: usize) {
        self.disassembled += &format!("LD V{} [I]\n", x);
    }
}