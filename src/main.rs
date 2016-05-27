const NUMBER_OF_REGISTERS: usize = 3;

#[derive(Debug)]
enum Instruction {
    Halt,
    Load {reg: usize, value: u16 },
    Swap {reg1: usize, reg2: usize, reg3: usize},
    Add {reg1: usize, reg2: usize, reg3: usize},
    Branch {offset: usize}
}

fn main() {
    let encoded_instructions = &[0x1110, 0x2100, 0x0, 0x0];

    cpu(encoded_instructions);
}

fn cpu(encoded_instructions: &[u16]) {
    let mut pc = 0;
    let mut reg_field = [0; NUMBER_OF_REGISTERS];

    loop {
      let encoded_instruction = fetch(pc, encoded_instructions);

      let decoded_instruction = decode(encoded_instruction);

      match decoded_instruction {
          Some(Instruction::Load {reg, value})
            => load(reg, value, &mut reg_field, &mut pc),
          Some(Instruction::Swap {reg1, reg2, reg3})
            => swap(reg1, reg2, reg3, &mut reg_field, &mut pc),
          Some(Instruction::Add {reg1, reg2, reg3})
            => add(reg1, reg2, reg3, &mut reg_field, &mut pc),
          Some(Instruction::Branch {offset})
            => branch(offset, &mut pc),
          Some(Instruction::Halt)
            => { halt(&reg_field); break; }
          None => break,
      }
    };
}

fn halt(register_field: &[u16]) {
    println!("{:?}", register_field[0]);
}

fn load(register: usize, value: u16, register_field: &mut[u16], pc: &mut usize) {
    register_field[register] = value;
    *pc += 1;
}

fn swap(reg1: usize, reg2: usize, reg3: usize, register_field: &mut[u16], pc: &mut usize) {
    register_field[reg3] = register_field[reg1];
    register_field[reg1] = register_field[reg2];
    register_field[reg2] = register_field[reg3];
    *pc += 1;
}

fn add(reg1: usize, reg2: usize, reg3: usize, register_field: &mut[u16], pc: &mut usize) {
    register_field[reg3] = register_field[reg1] + register_field[reg2];
    *pc += 1;
}

fn branch(offset: usize, pc: &mut usize){
    *pc -= offset;
}

fn fetch(pc: usize, instructions: &[u16]) -> u16 {
    instructions[pc]
}

fn decode(encoded_instruction: u16) -> Option<Instruction> {
    let operator = encoded_instruction >> 12;
    let reg1 = ((encoded_instruction >> 8) & 0xF) as usize;
    let reg2 = ((encoded_instruction >> 4) & 0xF) as usize;
    let reg3 = (encoded_instruction & 0xF) as usize;
    let offset = (encoded_instruction & 0xFFF) as usize;
    let value = encoded_instruction & 0xFF;

    match operator {
        0 => Some(Instruction::Halt),
        1 => Some(Instruction::Load {reg: reg1, value: value}),
        2 => Some(Instruction::Swap {reg1: reg1, reg2: reg2, reg3: reg3}),
        3 => Some(Instruction::Add {reg1: reg1, reg2: reg2, reg3: reg3}),
        4 => Some(Instruction::Branch {offset: offset}),
        _ => None
    }
}
