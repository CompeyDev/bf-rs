use crate::utils;

#[derive(Debug, Clone)]
pub struct BrainfuckInstance {
    vm: Vec<usize>,
    current_ptr: usize,
    next_ptr: usize,
    in_loop: bool,
}

pub struct Lexer {
    instance: BrainfuckInstance,
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

            if let None = self.vm.get(self.current_ptr) {
                self.vm.insert(self.current_ptr, 0);
            }

            let mut lexer_instance = Lexer::new((&*self).to_owned());

            let mutated_bf_instance = lexer_instance.parse(instr, instr_pos, in_loop);

            self.vm = mutated_bf_instance.vm;
            self.current_ptr = mutated_bf_instance.current_ptr;
            self.next_ptr = mutated_bf_instance.current_ptr;
            self.in_loop = mutated_bf_instance.in_loop;
        }
        println!("vm: {:#?}", self.vm);
    }
}

impl Lexer {
    pub fn new(instance: BrainfuckInstance) -> Self {
        return Self { instance };
    }

    pub fn parse(&mut self, instr: &str, instr_pos: usize, in_loop: bool) -> BrainfuckInstance {
        match instr {
            ">" => {
                if !in_loop {
                    self.instance.current_ptr += 1;
                    self.instance.next_ptr += 1;
                }
            }

            "<" => {
                if !in_loop {
                    self.instance.current_ptr -= 1;
                    self.instance.next_ptr -= 1;
                }
            }

            "+" => {
                if !in_loop {
                    self.instance.vm[self.instance.current_ptr] += 1;
                }
            }

            "-" => {
                if !in_loop {
                    self.instance.vm[self.instance.current_ptr] -= 1;
                }
            }

            &_ => utils::throw_err(
                "VM",
                format!("invalid instruction `{}` at ptr `{}`", instr, instr_pos).as_str(),
            ),
        }

        return self.instance.clone();
    }
}
