use std::mem;
use std::fmt::{self, Display};

pub type Value = u16;
const VALUE_SIZE: usize = mem::size_of::<Value>();

#[derive(Debug)]
pub enum Instruction {
    Nop(), // 0
    Push(Value), // 1
    Pop(), // 2
    Add(), // 3
    Print(), // 4
    Halt(), // 5
}

impl Instruction {
    fn size(&self) -> usize {
        match self {
            &Self::Nop() => 1,
            &Self::Push(_) => 1 + VALUE_SIZE,
            &Self::Pop() => 1,
            &Self::Add() => 1,
            &Self::Print() => 1,
            &Self::Halt() => 1,
        }
    } 
}

pub struct VM {
    code: Vec<u8>,
    ip: usize,
    stack: Vec<Value>,
    halt: bool,
}

#[derive(Debug)]
pub enum VmError {
    StackUnderflow{
        instr: Instruction,
        ip: usize
    },
}

impl Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::StackUnderflow{instr, ip} => write!(f, "stack underflow at instruction {:?} with index {}", instr, ip) 
        }
    }
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        Self {
            code: bytecode,
            ip: 0,
            stack: Vec::new(),
            halt: false,
        }
    }

    fn decode(&mut self) -> Instruction {
        match self.code[self.ip] {
            0 => {
                // Nop
                self.ip += 1;
                Instruction::Nop()
            }
            1 => {
                // Push
                self.ip += 1;
                let value =
                    Value::from_le_bytes(self.code[self.ip..self.ip + VALUE_SIZE].try_into().unwrap());
                self.ip += VALUE_SIZE;
                Instruction::Push(value)
            }
            2 => {
                // Pop
                self.ip += 1;
                Instruction::Pop()
            }
            3 => {
                // Add
                self.ip += 1;
                Instruction::Add()
            },
            4 => {
                // Print
                self.ip += 1;
                Instruction::Print()
            },
            5 => {
                // Halt
                self.ip += 1;
                Instruction::Halt()
            },
            _ => Instruction::Nop(),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), VmError> {
        println!("Execute: Instruction: {:?} Ip: {:?}", instruction, self.ip);      

        let previous_ip = self.ip - instruction.size();

        match instruction {
            Instruction::Nop() => {}
            Instruction::Push(value) => self.stack.push(value),
            Instruction::Pop() => match self.stack.pop() {
                None => return Err(VmError::StackUnderflow{instr: instruction, ip: previous_ip}),
                _ => {}
            },
            Instruction::Add() => {
                let left = match self.stack.pop() {
                    None => return Err(VmError::StackUnderflow{instr: instruction, ip: previous_ip}),
                    Some(v) => v,
                };
                let right = match self.stack.pop() {
                    None => return Err(VmError::StackUnderflow{instr: instruction, ip: previous_ip}),
                    Some(v) => v,
                };
                self.stack.push(left + right)
            }
            Instruction::Print() => match self.stack.pop() {
                Some(v) => println!("{}", v),
                None => println!("(empty)"),
            },
            Instruction::Halt() => self.halt = true
        };
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while self.ip < self.code.len() && !self.halt {
            let instruction = self.decode();
            self.execute(instruction)?;
        }  
        Ok(())
    }
}
