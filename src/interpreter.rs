use crate::opcodes::Opcodes;

use std::fs::File;
use std::io::{self, Read, Write};
use std::collections::HashMap;

const TOTAL_RAM_AMOUNT: u32 = 65536;
const RAM_END: u16 = (TOTAL_RAM_AMOUNT - 1) as u16;
const MAX_PROGRAM_OPS: u16 = 54000;

pub struct Interpreter {
    ram: [u8; TOTAL_RAM_AMOUNT as usize],
    program: Vec::<Opcodes>,
    jump_map: HashMap<usize, usize>,
    ptr: u32,
    pc: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            ram: [0; TOTAL_RAM_AMOUNT as usize],
            program: Vec::<Opcodes>::new(),
            jump_map: HashMap::<usize, usize>::new(),
            ptr: 0,
            pc: 0
        }
    }

    pub fn load_program(&mut self, program_path: &str) {
        let mut program_buffer = String::new();

        match File::open(program_path) {
            Ok(mut program_file) => {
                match program_file.read_to_string(&mut program_buffer) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Unable to read the given file !");
                        println!("(VERBOSE MESSAGE) {:?}", err);
                    },
                }
            },
            Err(err) => {
                println!("Unable to open the given file !");
                println!("(VERBOSE MESSAGE) {:?}", err);
            }
        }

        let mut op_count: usize = 0;
        for symbol in program_buffer.chars() {
            op_count += match symbol {
                '>' => {
                    self.program.push(Opcodes::IncPtr);
                    1
                },
                '<' => {
                    self.program.push(Opcodes::DecPtr);
                    1
                },
                '+' => {
                    self.program.push(Opcodes::IncByte);
                    1
                },
                '-' => {
                    self.program.push(Opcodes::DecByte);
                    1
                },
                '.' => {
                    self.program.push(Opcodes::WriteByte);
                    1
                },
                ',' => {
                    self.program.push(Opcodes::ReadByte);
                    1
                },
                '[' => {
                    self.program.push(Opcodes::LoopStart);
                    1
                },
                ']' => {
                    self.program.push(Opcodes::LoopEnd);
                    1
                },
                _ => 0
            };

            if op_count >= MAX_PROGRAM_OPS as usize {
                panic!("You've reached the max program ops limit !");
            }
        }
    }

    fn precompute_jumps(&mut self) {
        let mut stack: Vec<usize> = Vec::<usize>::new();

        let mut pc: usize = 0;

        while pc != self.program.len() {
            let op = self.program[pc];

            match op {
                Opcodes::LoopStart => stack.push(pc),
                Opcodes::LoopEnd => {
                    let target = stack.pop().unwrap();
                    self.jump_map.insert(target, pc);
                    self.jump_map.insert(pc, target);
                },
                _ => ()
            }

            pc += 1
        }
    }

    pub fn run(&mut self) {
        self.precompute_jumps();

        while self.pc != self.program.len() {
            let op = self.program[self.pc];

            match op {
                Opcodes::IncPtr => {
                    if self.ptr > RAM_END as u32 {self.ptr = 0;}
                    self.ptr = self.ptr.wrapping_add(1);
                },
                Opcodes::DecPtr => {
                    if self.ptr == 0 {self.ptr = RAM_END as u32 - 1;}
                    self.ptr = self.ptr.wrapping_sub(1);
                },
                Opcodes::IncByte => {
                    let prev_val = self.ram[self.ptr as usize];
                    self.ram[self.ptr as usize] = prev_val.wrapping_add(1);
                },
                Opcodes::DecByte => {
                    let prev_val = self.ram[self.ptr as usize];
                    self.ram[self.ptr as usize] = prev_val.wrapping_sub(1);
                },
                Opcodes::WriteByte => {
                    print!("{}", self.ram[self.ptr as usize] as char);
                    io::stdout().flush().unwrap();
                },
                Opcodes::ReadByte => {
                    let mut input: [u8; 1] = [0; 1];
                    io::stdin().read_exact(&mut input).expect("Unable to read stdin !");
                    self.ram[self.ptr as usize] = input[0];
                },
                Opcodes::LoopStart => {
                    if self.ram[self.ptr as usize] == 0 {
                        self.pc = *self.jump_map.get(&self.pc).unwrap();
                    }
                },
                Opcodes::LoopEnd => {
                    if self.ram[self.ptr as usize] != 0 {
                        self.pc = *self.jump_map.get(&self.pc).unwrap();
                    }
                },
            }

            self.pc += 1;
        }
    }
}