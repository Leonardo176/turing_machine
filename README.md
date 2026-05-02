
# TMSim: Turing Machine Simulator

## Introduction

This is a Turing machine simulator written in Rust.

It is designed to be as fast as possible on computing the instructions.

For now, you can do the following:

- Create a Turing Machine with a given tape and instructions
- Compute until it terminates (if it terminates!)

## Roadmap

This is what I want to implement:

- [x] State aliases
- [x] Custom symbols (for now only '0', 'b' symbols are supported)
- [x] Implement a builder pattern for creating a Turing Machine
- [ ] Import & Export a Turing Machine in JSON (there is already an example in input.json)
- [ ] Support for reverse computing (being able to reserve the computation)
- [x] Better error handling
