const NUMBER_OF_INSTRUCTIONS: usize = 4;

#[derive(Debug)]
enum InstructionSet {
    Halt,
    Load
}

fn main() {
    let mut ic = 0;
    let mut reg_field = [0; 2];
    let mut running = true;

    let instructions = [0x1410, 0xF0, 0xFFF, 0x0];

    while running {

      let instruction = fetch(&mut ic, instructions);

      let decoded_instruction = decode(instruction);

      running = false;

      println!("{:?}", decoded_instruction);
    };

}

fn fetch(ic: &mut usize, instructions: [i16; NUMBER_OF_INSTRUCTIONS]) -> i16 {
    let instruction = instructions[*ic];
    *ic += 1;
    instruction
}

fn decode(instruction: i16) -> InstructionSet {
    let instruction_number = instruction >> 12;
    let reg1 = (instruction << 4) >> 12;
    let reg2 = (instruction << 8) >> 12;
    let reg3 = (instruction << 12) >> 12;
    let value = (instruction << 8) >> 8;

    println!("{:?}",value );

    match instruction_number {
        0 => InstructionSet::Halt,
        1 => InstructionSet::Load,
        _ => InstructionSet::Load
    }
}
