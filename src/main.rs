mod common;
mod turing;

use turing::{Alias, Direction::*, Instruction, TuringMachine};

fn main() {
    let mut tm = TuringMachine::builder('b', vec!['b', '0'])
        .initial_state("start")
        .aliases(vec![Alias::new("start", 0), Alias::new("idk", 5)])
        .tape(0, &['b', '0', '0', '0', '0', '0'])
        .instructions(vec![
            // This instructions move the "given" string after the first blank
            // Example: given this tape (b00000) where the current index is on the first blank
            // This is the result (bbbbbb00000b) and now the current index is the last b
            Instruction::new("start", 'b', "start+1", 'b', Right),
            Instruction::new("start+1", 'b', 20, 'b', Right),
            Instruction::new("start+1", '0', "start+2", '0', Right),
            Instruction::new(2, 'b', 3, 'b', Right),
            Instruction::new(2, '0', 2, '0', Right),
            Instruction::new(3, 'b', 4, '0', Left),
            Instruction::new(3, '0', 3, '0', Right),
            Instruction::new(4, 'b', "idk", 'b', Left),
            Instruction::new(4, '0', 4, '0', Left),
            Instruction::new("idk", 'b', 6, 'b', Right),
            Instruction::new(5, '0', 5, '0', Left),
            Instruction::new(6, '0', 1, 'b', Right),
        ])
        .build()
        .unwrap();

    tm.compute();

    println!("{}", tm.get_tape());
}
