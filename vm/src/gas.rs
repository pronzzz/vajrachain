use crate::bytecode::OpCode;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GasError {
    #[error("Out of gas")]
    OutOfGas,
}

pub struct GasMeter {
    pub limit: u64,
    pub consumed: u64,
}

impl GasMeter {
    pub fn new(limit: u64) -> Self {
        Self { limit, consumed: 0 }
    }

    pub fn consume(&mut self, amount: u64) -> Result<(), GasError> {
        if self.consumed + amount > self.limit {
            return Err(GasError::OutOfGas);
        }
        self.consumed += amount;
        Ok(())
    }

    pub fn cost(&self, op: OpCode) -> u64 {
        match op {
            OpCode::STOP => 0,
            OpCode::ADD => 3,
            OpCode::SUB => 3,
            OpCode::MUL => 5,
            OpCode::DIV => 5,
            OpCode::PUSH(_) => 2,
            OpCode::POP => 2,
            OpCode::DUP => 3,
            OpCode::SWAP => 3,
            OpCode::LOAD => 10,  // Memory access expensive
            OpCode::STORE => 10, // Memory access expensive
            OpCode::JUMP => 8,
            OpCode::JUMPI => 10,
        }
    }
}
