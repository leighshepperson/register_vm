type RegisterFile = [u16; 3];

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Halt,
    Load { destination: usize, value: u16 },
    Mov { reg1: usize, reg2: usize, reg3: usize },
    Add { destination: usize, source: usize },
    Branch { offset: usize }
}

impl Instruction {
    fn decode(encoded_instruction: u16) -> Option<Self> {
        let operator = encoded_instruction >> 12;
        let reg1 = ((encoded_instruction >> 8) & 0xF) as usize;
        let reg2 = ((encoded_instruction >> 4) & 0xF) as usize;
        let reg3 = (encoded_instruction & 0xF) as usize;
        let offset = (encoded_instruction & 0xFFF) as usize;
        let value = encoded_instruction & 0xFF;

        match operator {
            0 => Some(Instruction::Halt),
            1 => Some(Instruction::Load { destination: reg1, value: value }),
            2 => Some(Instruction::Mov { reg1: reg1, reg2: reg2, reg3: reg3 }),
            3 => Some(Instruction::Add { destination: reg1, source: reg2 }),
            4 => Some(Instruction::Branch { offset: offset }),
            _ => None,
        }
    }

    fn execute(&self, registers: &mut [u16], ic: &mut usize) -> bool {
        match *self {
            Instruction::Load { destination, value } => {
                load(destination, value, registers);
            },
            Instruction::Mov { reg1, reg2, reg3 } => {
                mov(reg1, reg2, reg3, registers);
            },
            Instruction::Add { destination, source } => {
                add(destination, source, registers);
            },
            Instruction::Branch { offset } => {
                branch(offset, ic);
            },
            Instruction::Halt => {
                halt(registers);
                return false;
            },
        }

        true
    }
}

fn halt(register_file: &[u16]) {
    println!("{:?}", register_file[0]);
}

fn load(destination: usize, value: u16, register_file: &mut [u16]) {
    register_file[destination] = value;
}

fn mov(reg1: usize, reg2: usize, reg3: usize, register_file: &mut [u16]) {
    register_file[reg3] = register_file[reg1];
    register_file[reg1] = register_file[reg2];
    register_file[reg2] = register_file[reg3];
}

fn add(destination: usize, source: usize, register_file: &mut [u16]) {
    register_file[destination] = register_file[destination] + register_file[source];
}

fn branch(offset: usize, ic: &mut usize) {
    *ic -= offset - 1;
}

struct Program<'a> {
    instructions: &'a [u16],
}

impl<'a> Program<'a> {
    fn fetch(&self, ic: usize) -> u16 {
        self.instructions[ic]
    }
}

fn cpu(program: Program) {
    let mut ic = 0;
    let mut registers = RegisterFile::default();

    loop {
        let encoded_instruction = program.fetch(ic);
        let decoded_instruction = Instruction::decode(encoded_instruction);

        match decoded_instruction {
            Some(instr) => {
                if !instr.execute(&mut registers, &mut ic) { break }
            }
            None => break,
        }

        ic += 1;
    }
}

fn main() {
    let encoded_instructions = Program { instructions: &[0x1110, 0x2100, 0x3010, 0x0] };

    cpu(encoded_instructions);
}