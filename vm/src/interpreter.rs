use crate::bytecode::{OpCode, Program};
use crate::gas::{GasError, GasMeter};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VMError {
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid jump destination")]
    InvalidJump,
    #[error("Gas error: {0}")]
    GasError(#[from] GasError),
}

pub struct VM {
    stack: Vec<u64>,
    #[allow(dead_code)]
    memory: Vec<u8>,
    pc: usize,
    gas_meter: GasMeter,
}

impl VM {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            stack: Vec::with_capacity(1024),
            memory: vec![0; 1024], // Simple linear memory 1KB
            pc: 0,
            gas_meter: GasMeter::new(gas_limit),
        }
    }

    pub fn execute(&mut self, program: &Program) -> Result<Option<u64>, VMError> {
        self.pc = 0;
        loop {
            if self.pc >= program.code.len() {
                break;
            }

            let op = program.code[self.pc];

            // Gas Check
            let cost = self.gas_meter.cost(op);
            self.gas_meter.consume(cost)?;

            match op {
                OpCode::STOP => return Ok(self.stack.last().copied()), // Return top of stack
                OpCode::PUSH(val) => self.stack.push(val),
                OpCode::POP => {
                    self.stack.pop().ok_or(VMError::StackUnderflow)?;
                }
                OpCode::ADD => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    self.stack.push(a.wrapping_add(b));
                }
                OpCode::SUB => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    self.stack.push(a.wrapping_sub(b));
                }
                OpCode::MUL => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    self.stack.push(a.wrapping_mul(b));
                }
                OpCode::DIV => {
                    let b = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    if b == 0 {
                        return Err(VMError::DivisionByZero);
                    }
                    let a = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    self.stack.push(a.wrapping_div(b));
                }
                OpCode::JUMP => {
                    let dest = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    if dest as usize >= program.code.len() {
                        return Err(VMError::InvalidJump);
                    }
                    self.pc = dest as usize;
                    continue; // Skip pc increment
                }
                OpCode::JUMPI => {
                    let dest = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    let condition = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                    if condition != 0 {
                        if dest as usize >= program.code.len() {
                            return Err(VMError::InvalidJump);
                        }
                        self.pc = dest as usize;
                        continue;
                    }
                }
                // Simplified Memory/Other ops skipped for brevity in this step
                _ => {}
            }

            self.pc += 1;
        }
        Ok(self.stack.last().copied())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let mut vm = VM::new(100);
        let prog = Program {
            code: vec![
                OpCode::PUSH(10),
                OpCode::PUSH(20),
                OpCode::ADD,
                OpCode::STOP,
            ],
        };
        let res = vm.execute(&prog).unwrap();
        assert_eq!(res, Some(30));
    }

    #[test]
    fn test_gas_limit() {
        let mut vm = VM::new(5); // Not enough gas
        let prog = Program {
            code: vec![
                OpCode::PUSH(10),
                OpCode::PUSH(20),
                OpCode::ADD, // Cost 3 + 2 + 2 = 7 > 5
            ],
        };
        let res = vm.execute(&prog);
        assert_eq!(res, Err(VMError::GasError(GasError::OutOfGas)));
    }

    #[test]
    fn test_jump() {
        let mut vm = VM::new(100);
        let prog = Program {
            code: vec![
                OpCode::PUSH(4), // Jump dest index
                OpCode::JUMP,
                OpCode::PUSH(999), // Should skip this
                OpCode::STOP,      // PC 3
                OpCode::PUSH(42),  // PC 4 (Dest)
                OpCode::STOP,
            ],
        };
        // Note: my JUMP implementation expects dest on stack.
        // Index mapping:
        // 0: PUSH(4)
        // 1: JUMP
        // 2: PUSH(999)
        // 3: STOP
        // 4: PUSH(42)
        // 5: STOP
        let res = vm.execute(&prog).unwrap();
        assert_eq!(res, Some(42));
    }
}
