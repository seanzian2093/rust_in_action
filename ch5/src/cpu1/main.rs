// Emulate a chip-8 cpu
struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();
        // Deconstruct the opcode
        // opcode is a u16
        // when written in hex, e.g.,`0x73EE` each of last four character is 2 bytes
        // First byte is high byte, and second low byte
        // Each byte is made of high nibble, and low nibble

        // For adder
        // Op group
        let c = ((opcode & 0xF000) >> 12) as u8;
        // CPU register
        let x = ((opcode & 0x0F00) >> 8) as u8;
        // CPU register
        let y = ((opcode & 0x00F0) >> 4) as u8;
        // Op subgroup
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c,x,y,d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

pub fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);
    println!("\n5 + 10 = {}", cpu.registers[0]);
}