use crate::utils;
use ascii::AsciiChar;
use std::{iter::FromIterator, str};
use utils::AnyMap as HashMap;

#[derive(Debug, Clone)]
pub struct BrainfuckInstance {
    vm: Vec<usize>,
    current_ptr: u32,
    next_ptr: u32,
    in_loop: bool,
    loop_meta: Option<BrainfuckLoop>,
}

#[derive(Debug, Clone)]
struct BrainfuckLoop {
    current_loop: Option<Vec<usize>>,
    loop_start: Option<usize>,
    loop_end: Option<usize>,
}

pub struct Lexer;

pub struct Instructions {
    instructions: HashMap<Instruction>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum Instruction {
    MovR,
    MovL,
    Incr,
    Decr,
    LoopEnter,
    LoopEnd,
    Print,
}

impl BrainfuckInstance {
    pub fn new() -> Self {
        return Self {
            vm: Vec::new(),
            current_ptr: 0,
            next_ptr: 1,
            in_loop: false,
            loop_meta: None,
        };
    }

    pub fn load_string(&mut self, input: String) {
        let mut in_loop: bool;

        for (instr_pos, instr_char) in input.chars().enumerate() {
            in_loop = self.in_loop;

            let instr_string = instr_char.to_string();
            let instr = instr_string.as_str();

            if let None = self.vm.get(self.current_ptr as usize) {
                self.vm.insert(self.current_ptr as usize, 0);
            }

            let mut lexer_instance = Lexer::new();

            let cloned_instance = &mut self.clone();
            let (mutated_in_loop, mutated_bf_instance) =
                lexer_instance.parse(instr, instr_pos)(in_loop, cloned_instance);

            let new_bf_instance = mutated_bf_instance.clone();

            self.vm = new_bf_instance.vm;
            self.current_ptr = new_bf_instance.current_ptr;
            self.next_ptr = new_bf_instance.current_ptr;
            self.loop_meta = new_bf_instance.loop_meta;
            self.in_loop = mutated_in_loop;

            #[cfg(debug_assertions)]
            println!("{instr_pos}: {instr}, {:#?}", self.loop_meta);
        }

        #[cfg(debug_assertions)]
        println!("current vm structure: {:#?}", self.vm);
    }
}

impl Lexer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn parse(
        &mut self,
        instr: &str,
        instr_pos: usize,
    ) -> fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance) {
        let instr_type = match instr {
            ">" => Instruction::MovR,
            "<" => Instruction::MovL,
            "+" => Instruction::Incr,
            "-" => Instruction::Decr,
            "[" => Instruction::LoopEnter,
            "]" => Instruction::LoopEnd,
            "." => Instruction::Print,
            &_ => utils::throw_err(
                "VM",
                format!("invalid instruction `{}` at ptr `{}`", instr, instr_pos).as_str(),
            ),
        };

        let mut instructions = Instructions::new();

        instructions.populate();

        let instr_handler = instructions.get_handler(instr_type).unwrap();

        return instr_handler.clone();
    }
}

impl Instructions {
    pub fn new() -> Self {
        return Self {
            instructions: HashMap::new(),
        };
    }

    pub fn populate(&mut self) {
        self.register(Instruction::MovR, |in_loop, instance| {
            if !in_loop {
                instance.current_ptr += 1;
                instance.next_ptr += 1;
            }

            return (in_loop, &*instance);
        });

        self.register(Instruction::MovL, |in_loop, instance| {
            if instance.current_ptr != 0 {
                instance.current_ptr -= 1;
                instance.next_ptr -= 1;
            } else {
                utils::throw_err("VM", "current memory chunk at 0");
            }

            return (in_loop, &*instance);
        });

        self.register(Instruction::Incr, |in_loop, instance| {
            instance.vm[instance.current_ptr as usize] += 1;

            return (in_loop, &*instance);
        });

        self.register(Instruction::Decr, |in_loop, instance| {
            let current_mem_chunk = instance.vm[instance.current_ptr as usize];

            if current_mem_chunk != 0 {
                instance.vm[instance.current_ptr as usize] -= 1;
            } else {
                utils::throw_err("VM", "current memory chunk at 0");
            }

            return (in_loop, &*instance);
        });

        self.register(Instruction::LoopEnter, |_in_loop, instance| {
            let current_mem_chunk = instance.vm[instance.current_ptr as usize];

            if current_mem_chunk != 0 {
                instance.in_loop = true;
                instance.loop_meta = Some(BrainfuckLoop {
                    current_loop: Some(Vec::new()),
                    loop_start: Some(instance.current_ptr as usize),
                    loop_end: None,
                });
            }

            return (instance.in_loop, &*instance);
        });

        self.register(Instruction::LoopEnd, |in_loop, instance| {
            let current_mem_chunk = instance.vm[instance.current_ptr as usize];

            if current_mem_chunk == 0 {
                instance.in_loop = false;
                instance.loop_meta = None;
            }

            if in_loop {
                let loop_meta = instance.loop_meta.as_mut().unwrap();

                loop_meta.loop_end = Some(instance.current_ptr as usize);
                loop_meta.current_loop = Some(Vec::from_iter(
                    instance.vm[loop_meta.loop_start.unwrap()..loop_meta.loop_end.unwrap()]
                        .iter()
                        .cloned(),
                ));
            }

            return (instance.in_loop, &*instance);
        });

        self.register(Instruction::Print, |in_loop, instance| {
            let current_mem_chunk = instance.vm[instance.current_ptr as usize];

            let char_code = current_mem_chunk as u8;
            let encoded_str = match AsciiChar::from_ascii(char_code) {
                Ok(str) => str,
                Err(_) => {
                    utils::throw_err(
                        "VM",
                        format!("invalid char code `{}`", current_mem_chunk).as_str(),
                    );
                }
            };

            print!("{encoded_str}");

            return (in_loop, &*instance);
        });
    }

    pub fn register(
        &mut self,
        instr: Instruction,
        implementation: fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance),
    ) {
        self.instructions.insert(instr, implementation);
    }

    pub fn get_handler(
        &self,
        instr: Instruction,
    ) -> Option<&fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance)> {
        return self
            .instructions
            .get::<fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance)>(instr);
    }
}
