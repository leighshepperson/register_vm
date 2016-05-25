#[derive(Copy, Clone)]
struct Register<T> {
    value: T
}

impl<T> Register<T> {
    fn write(&mut self, value: T) {self.value = value; }
    fn read(self) -> T { self.value }
}

fn main() {
    let mut instruction_counter: Register<usize> = Register { value: 0 };
    let mut registers: [Register<i16>; 2] = [Register { value: 0 }; 2];
    let mut running = true;

    let stored_instructions = &[0xAF4, 33, 13];

    while running {
        println!("{:?}", instruction_counter.read() );
        instruction_counter.write(1);
        let instruction = fetch(stored_instructions, &mut instruction_counter);
        println!("{:?}", instruction);

        running = false;
    }
}

fn fetch(stored_instructions: &[i32], instruction_counter: &mut Register<usize>) -> i32 {
    let old = instruction_counter.read();
    instruction_counter.write(old + 1);
    stored_instructions[old]
}
