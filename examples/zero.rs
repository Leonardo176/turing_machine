use turing_machine::{Direction::*, Instruction, TuringMachine};

// This Turing Machine calculates the zero(x) function
// Example:
// Tape: "b0000"
// Output: "b0000b0"
// The end position of the tape is on the second 'b'
fn main() {
    let mut tm = TuringMachine::builder('b', vec!['b', '0'])
        .tape(0, &['b', '0', '0', '0', '0'])
        .instructions(vec![
            Instruction::new(0, 'b', 1, 'b', Right),
            Instruction::new(1, '0', 1, '0', Right),
            Instruction::new(1, 'b', 2, 'b', Right),
            Instruction::new(2, 'b', 3, '0', Left),
        ])
        .build()
        .unwrap();

    tm.compute();

    println!("{}", tm.get_tape());
}
