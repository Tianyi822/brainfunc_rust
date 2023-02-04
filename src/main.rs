use std::{env, error, fs, io};
use std::io::{Read, Write};

use opcode::Code;

use crate::opcode::Opcode;

mod opcode;

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            stack: vec![0; 1],
        }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0; // Program counter
        let mut sp = 0; // Stack pointer

        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                Opcode::SHL => sp = if sp == 0 { 0 } else { sp - 1 },
                Opcode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0)
                    }
                }
                Opcode::ADD => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                }
                Opcode::PUTCHAR => {
                    io::stdout().write_all(&[self.stack[sp]])?;
                }
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                Opcode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }
            pc += 1;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(&args[1])?;

    let mut interpreter = Interpreter::new();
    interpreter.run(data).expect("");

    Ok(())
}
