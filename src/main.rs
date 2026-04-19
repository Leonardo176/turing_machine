mod turing;

use turing::{Alias, Direction::*, Instruction, State::Int, Symbol::*, TuringMachine, alias};

fn main() {
    let mut tm = TuringMachine::builder()
        .aliases(vec![Alias::new("start", 0), Alias::new("idk", 5)])
        .tape(0, &[Blank, Zero, Zero, Zero, Zero, Zero])
        .instructions(vec![
            // This instructions move the "given" string after the first blank
            // Example: given this tape (b00000) where the current index is on the first blank
            // This is the result (bbbbbb00000b) and now the current index is the last b
            Instruction::new(alias("start"), Blank, Int(1), Blank, Right),
            Instruction::new(Int(1), Blank, Int(20), Blank, Right),
            Instruction::new(Int(1), Zero, Int(2), Zero, Right),
            Instruction::new(Int(2), Blank, Int(3), Blank, Right),
            Instruction::new(Int(2), Zero, Int(2), Zero, Right),
            Instruction::new(Int(3), Blank, Int(4), Zero, Left),
            Instruction::new(Int(3), Zero, Int(3), Zero, Right),
            Instruction::new(Int(4), Blank, alias("idk"), Blank, Left),
            Instruction::new(Int(4), Zero, Int(4), Zero, Left),
            Instruction::new(alias("idk"), Blank, Int(6), Blank, Right),
            Instruction::new(Int(5), Zero, Int(5), Zero, Left),
            Instruction::new(Int(6), Zero, Int(1), Blank, Right),
        ])
        .build();

    tm.compute();

    println!("{}", tm.get_tape());
}
