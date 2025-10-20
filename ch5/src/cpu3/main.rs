

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    // To build functions, we need to implement -
    // CALL opcode `0x2nnn` where `nnn` is memory address
    // RETURN opcode `0x00EE` sets `position_in_memory` to address of previous CALL opcode
    stack: [u16; 16],
    // `usize` is easier for indexing
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        op_byte1 << 8 | op_byte2
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

            let nnn = opcode & 0x0FFF;
            match (c,x,y,d) {
                (0,0,0,0) => { return; },
                (0,0,0xE,0xE) => self.ret(),
                // opcode like `0x2100` will call function at `0x100` in mem
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    // Call a function which resides at `addr` in memory
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        // Save current position in stack
        stack[sp] = self.position_in_memory as u16;
        // Move one step from saved position to prevent from being overwritten?
        self.stack_pointer += 1;
        // Set position to the target address - where a function resides
        // So next operation will be calling the function
        self.position_in_memory = addr as usize;
    }

    // Return from a function call
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        // Move one step back since we incremented it in `call`
        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        // Move the position back to where before calling the function
        self.position_in_memory = call_addr as usize;
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
}

pub fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // Call function at `0x100` - run op `0x8014` twice and return
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    // Call function at `0x100` - run op `0x8014` twice and return
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    // Terminate/Halt
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    // op `0x8014`
    mem[0x100] = 0x80; mem[0x101] = 0x14;
    // op `0x8014`
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    // return
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}