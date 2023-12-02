# DFA Simulation Engine

## Overview

This documentation provides an overview and explanation of the DFA (Deterministic Finite Automaton) code. The code is organized into several modules, each serving a specific purpose in simulating a system with multiple state machines.

## Modules

### `stream`

The `stream` module contains the main functionality for managing and simulating multiple state machines using the DFA approach.

#### Files

- **`globals.rs`**: Defines the global parameters used in the simulation.

- **`machine.rs`**: Defines the structure of the state machines.

- **`dsl.rs`**: Implements a DSL (Domain-Specific Language) for reading and interpreting machine configurations.

### `machine`

The `machine` module contains components related to the state machine abstraction.

#### Files

- **`abstract_machine.rs`**: Defines the `AbstractMachine` struct representing the abstract structure of a state machine.

- **`dfa.rs`**: Implements the `DFA` (Deterministic Finite Automaton) struct representing an individual state machine.

- **`state.rs`**: Defines the `State` struct representing the possible states of a state machine.

### `reader`

The `reader` module provides functionality to read and interpret machine configurations from a DSL.

#### Files

- **`global_reader.rs`**: Implements a reader for global parameters.

- **`machine_reader.rs`**: Implements a reader for the state machine configurations.

- **`reader.rs`**: Combines the global and machine readers to read the entire machine configuration.

## Main Application (`main.rs`)

The `main.rs` file serves as the entry point for the application. It parses command-line arguments, reads machine configurations from a DSL file, and initiates the simulation.

#### Key Functions

- **`main`**: The main function of the application, responsible for coordinating the simulation.

- **`StreamEngine::new`**: Initializes a new `StreamEngine` with global and machine configurations.

- **`StreamEngine::multi_stream`**: Drives the simulation by managing a priority queue of state machines.

## How to Use

1. **Install Rust**: Make sure you have Rust installed on your system.

2. **Run the Application**: Execute the application with the path to the DSL file as a command-line argument.

   ```bash
   cargo run --release path/to/your/dsl_file.dsl
   ```
   Example
    ```bash
   cargo run -- dsl/test.lua
    ```
   The heartbeats are generated and written to `output.json`.



### Dependencies
`rlua`: A Rust crate for working with the Lua scripting language.

`double_priority_queue`: A priority queue implementation.

`rand`: A crate for random number generation.
