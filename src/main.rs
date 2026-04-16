mod turing;

use turing::{Direction::*, Instruction, Symbol::*, TuringMachine};

fn main() {
    // Creating a Turing Machine from a tape
    let mut tm = TuringMachine::from_tape(
        0,
        &[Blank, Zero, Zero, Zero, Zero, Zero],
        vec![
            // This instructions move the "given" string after the first blank
            // Example: given this tape (b00000) where the current index is on the first blank
            // This is the result (bbbbbb00000b) and now the current index is the last b
            Instruction::new(0, Blank, 1, Blank, Right),
            Instruction::new(1, Blank, 20, Blank, Right),
            Instruction::new(1, Zero, 2, Zero, Right),
            Instruction::new(2, Blank, 3, Blank, Right),
            Instruction::new(2, Zero, 2, Zero, Right),
            Instruction::new(3, Blank, 4, Zero, Left),
            Instruction::new(3, Zero, 3, Zero, Right),
            Instruction::new(4, Blank, 5, Blank, Left),
            Instruction::new(4, Zero, 4, Zero, Left),
            Instruction::new(5, Blank, 6, Blank, Right),
            Instruction::new(5, Zero, 5, Zero, Left),
            Instruction::new(6, Zero, 1, Blank, Right),
        ],
    );

    tm.compute();

    println!("{}", tm.get_tape());
}
