use turing_machine::*;

use Direction::*;

/*
 * This Turing Machine makes a copy of the input string
 * It ends on the 'b' before the copied string
 * Example:
 * Tape: 'b000000'
 * After exec: 'b000000b000000'
 */
fn main() {
    let mut tm = TuringMachine::builder('b', vec!['b', '0'])
        .aliases(vec![Alias::new("start", 0), Alias::new("copy", 10)])
        .initial_state("start")
        .tape(0, &['b'])
        .instructions(vec![
            Instruction::new("start", 'b', "copy", 'b', Right),
            Instruction::new("copy", '0', "copy+1", 'b', Right),
            Instruction::new("copy+1", '0', "copy+1", '0', Right),
            Instruction::new("copy+1", 'b', "copy+2", 'b', Right),
            Instruction::new("copy+2", 'b', "copy+3", '0', Left),
            Instruction::new("copy+2", '0', "copy+2", '0', Right),
            Instruction::new("copy+3", 'b', "copy+4", 'b', Left),
            Instruction::new("copy+3", '0', "copy+3", '0', Left),
            Instruction::new("copy+4", '0', "copy+4", '0', Left),
            Instruction::new("copy+4", 'b', "copy", '0', Right),
        ])
        .build()
        .unwrap();

    println!("{}", tm.get_tape());

    while tm.compute_single().is_some() {
        println!("{}", tm.get_tape());
    }
}
