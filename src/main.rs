mod turing;

use turing::{Alias, Direction::*, Instruction, State::Int, TuringMachine, alias};

fn main() {
    let mut tm = TuringMachine::builder()
        .aliases(vec![Alias::new("start", 0), Alias::new("idk", 5)])
        .tape(0, &['b', '0', '0', '0', '0', '0'])
        .instructions(vec![
            // This instructions move the "given" string after the first blank
            // Example: given this tape (b00000) where the current index is on the first blank
            // This is the result (bbbbbb00000b) and now the current index is the last b
            Instruction::new(alias("start"), 'b', Int(1), 'b', Right),
            Instruction::new(Int(1), 'b', Int(20), 'b', Right),
            Instruction::new(Int(1), '0', Int(2), '0', Right),
            Instruction::new(Int(2), 'b', Int(3), 'b', Right),
            Instruction::new(Int(2), '0', Int(2), '0', Right),
            Instruction::new(Int(3), 'b', Int(4), '0', Left),
            Instruction::new(Int(3), '0', Int(3), '0', Right),
            Instruction::new(Int(4), 'b', alias("idk"), 'b', Left),
            Instruction::new(Int(4), '0', Int(4), '0', Left),
            Instruction::new(alias("idk"), 'b', Int(6), 'b', Right),
            Instruction::new(Int(5), '0', Int(5), '0', Left),
            Instruction::new(Int(6), '0', Int(1), 'b', Right),
        ])
        .build();

    tm.compute();

    println!("{}", tm.get_tape());
}
