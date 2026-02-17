use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpCode {
    STOP,
    ADD,
    SUB,
    MUL,
    DIV,
    PUSH(u64), // Simple: only pushing u64 for now
    POP,
    DUP,
    SWAP,
    LOAD,  // Load from Memory
    STORE, // Store to Memory
    JUMP,
    JUMPI, // Jump if non-zero
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub code: Vec<OpCode>,
}
