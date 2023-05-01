use crate::utils;
use utils::AnyMap as HashMap;

#[derive(Debug, Clone)]
pub struct BrainfuckInstance {
    vm: Vec<usize>,
    current_ptr: u32,
    next_ptr: u32,
    in_loop: bool,
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
}

impl BrainfuckInstance {
    pub fn new() -> Self {
        return Self {
            vm: Vec::new(),
            current_ptr: 0,
            next_ptr: 1,
            in_loop: false,
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
            let (mutated_in_loop, mutated_bf_instance) = lexer_instance.parse(instr, instr_pos)(in_loop, cloned_instance);

            let new_bf_instance = mutated_bf_instance.clone();

            self.vm = new_bf_instance.vm;
            self.current_ptr = new_bf_instance.current_ptr;
            self.next_ptr = new_bf_instance.current_ptr;
            self.in_loop = mutated_in_loop;
        }
        println!("vm: {:#?}", self.vm);
    }
}

impl Lexer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn parse(&mut self, instr: &str, instr_pos: usize) -> fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance) {
        let instr_type = match instr {
            ">" => Instruction::MovR,
            "<" => Instruction::MovL,
            "+" => Instruction::Incr,
            "-" => Instruction::Decr,
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

            return (in_loop, &*instance)
        });

        self.register(Instruction::MovL, |in_loop, instance| {
            if !in_loop && instance.current_ptr != 0 {
                instance.current_ptr -= 1;
                instance.next_ptr -= 1;
            } else {
                utils::throw_err("VM", "current pointer at 0 or in loop");
            }

            return (in_loop, &*instance)
        });

        self.register(Instruction::Incr, |in_loop, instance| {
            if !in_loop {
                instance.vm[instance.current_ptr as usize] += 1;
            }

            return (in_loop, &*instance)
        });

        self.register(Instruction::Decr, |in_loop, instance| {
            let current_mem_chunk = instance.vm[instance.current_ptr as usize];
            
            if !in_loop && current_mem_chunk != 0 {
                instance.vm[instance.current_ptr as usize] -= 1; // ERROR: ATTEMPT TO SUBTRACT WITH OVERFLOW
            } else {
                utils::throw_err("VM", "current pointer at 0 or in loop");
            }

            return (in_loop, &*instance)
        });
    }

    pub fn register(&mut self, instr: Instruction, implementation: fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance)) {
        self.instructions.insert(instr, implementation);

    }

    pub fn get_handler(&self, instr: Instruction) -> Option<&fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance)> {
        return self.instructions.get::<fn(bool, &mut BrainfuckInstance) -> (bool, &BrainfuckInstance)>(instr);
    }
}
