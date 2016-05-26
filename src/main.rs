const NUMBER_OF_INSTRUCTIONS: usize = 4;

#[derive(Debug)]
enum InstructionSet {
    Halt,
    Load {reg: i16, value: i16 },
    Mov {reg1: i16, reg2: i16},
    Add {reg1: i16, reg2: i16, reg3: i16}
}

fn main() {
    let mut ic = 0;
    let mut reg_field = [0; 2];
    let mut running = true;

    let instructions = [0x3410, 0xF0, 0xFFF, 0x0];

    while running {

      let instruction = fetch(ic, instructions);

      let decoded_instruction = decode(instruction);

      running = false;

      println!("{:?}", decoded_instruction);
    };

}

fn fetch(ic: usize, instructions: [i16; NUMBER_OF_INSTRUCTIONS]) -> i16 {
    instructions[ic]
}

fn decode(instruction: i16) -> Option<InstructionSet> {
    let instruction_number = instruction >> 12;
    let reg1 = (instruction >> 8) & 0xF;
    let reg2 = (instruction >> 4) & 0xF;
    let reg3 = instruction & 0xF;
    let value = instruction & 0xFF;

    match instruction_number {
        0 => Some(InstructionSet::Halt),
        1 => Some(InstructionSet::Load {reg: reg1 , value: value}),
        2 => Some(InstructionSet::Mov {reg1: reg1 , reg2: reg2}),
        3 => Some(InstructionSet::Add {reg1: reg1, reg2: reg2, reg3: reg3}),
        _ => None
    }
}
