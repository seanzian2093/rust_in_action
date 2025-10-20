struct CPU {
    // Suppose we have 16 register so conveniently a hexadecimal can handle it
    // Use last register `0xF` as `carry flag` - when set, indicating operation overflowed u8 size
    registers: [u8; 16],
    // program counter in other text books
    position_in_memory: usize,
    // chip-8 has only 4096 bytes memory, i.e., 1000 in hexadecimal
    // in reality of chip-8, first 100 bytes are reserved for system
    memory: [u8; 0x1000],

}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        // Retrieve 2 u8 value from memory, successively
        // Cast to u16 for shifting
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // Shift `op_byte1` left by 8 bits to make room for `op_byte2`
        op_byte1 << 8 | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            // Move position after read
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c,x,y,d) {
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                // Terminate the program when read `0x0000`, i.e., empty memory
                (0,0,0,0) => { return; }
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
}

pub fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    // opcode `0x8014` - add register 1 to register 0
    mem[0] = 0x80;mem[1] = 0x14;
    // opcode `0x8024` - add register 2 to register 0
    mem[2] = 0x80;mem[3] = 0x24;
    // opcode `0x8034` - add register 3 to register 0
    mem[4] = 0x80;mem[5] = 0x34;
    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("\n5 + 10 + 10 + 10 = {}", cpu.registers[0]);

}
