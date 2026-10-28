// CHIP-8 opcodes are 4 nibbels, but since there is no 4 bit type in rust we will use u16 for 4 nibbles
// each opcode is made up of two bytes -> high byte and low byte and each byte is made up of two nibbles (high and low)

struct CPU {
    registers: [u8; 16],
    memory: [u8; 0x1000],      // CHIP-8 has 4096 bytes of RAM
    position_in_memory: usize, // program counter
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
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8; // opcode group (high byte, high nibble)
            let x = ((opcode & 0x0F00) >> 8) as u8; // cpu register (high byte, low nibble)
            let y = ((opcode & 0x00F0) >> 4) as u8; // cpu register (low byte, high nibble)
            let d = ((opcode & 0x000F) >> 0) as u8; // opcode subgroup (low byte, low nibble)

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode: {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // last register is used as a carry flag
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    // steps to perform addition
    // 1) initialize CPU
    // 2) load u8 values to register
    // 3) load addition opcode into current_operation
    // 4) perform operation

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
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 = {}", cpu.registers[0]);
}
