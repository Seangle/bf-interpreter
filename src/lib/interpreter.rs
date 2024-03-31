use std::io::{self, BufRead};

pub struct BfInterpreter {
    mem: Vec<u8>,
    ptr: usize,
}

impl BfInterpreter {
    pub fn new() -> BfInterpreter {
        BfInterpreter {
            mem: Vec::new(),
            ptr: 0,
        }
    }

    pub fn memory_size(&self) -> usize {
        self.mem.len()
    }

    pub fn interpret(&mut self, source: &str) {
        let len = source.chars().count();
        let mut stack: Vec<usize> = Vec::new();
        let mut idx = 0;
        // For bf comments - how many braces deep do we not execute.
        let mut stack_size = 0;

        if self.mem.len() == 0 {
            self.mem.push(0)
        }

        while idx < len {
            let c = source.chars().nth(idx).unwrap();

            match (c, stack_size) {
                ('[', _) => {
                    stack.push(idx);

                    if self.mem[self.ptr] == 0 {
                        stack_size += 1;
                    }
                }
                (']', _) => {
                    let addr = stack.pop().unwrap();
                    if stack_size != 0 {
                        stack_size -= 1;
                    }
                    if self.mem[self.ptr] != 0 {
                        idx = addr - 1
                    }
                }
                ('<', 0) => self.ptr -= 1,
                ('>', 0) => {
                    self.ptr += 1;

                    if self.ptr >= self.mem.len() {
                        self.mem.push(0)
                    }
                },
                ('-', 0) => self.mem[self.ptr] -= 1,
                ('+', 0) => self.mem[self.ptr] += 1,
                ('.', 0) => print!("{}", self.mem[self.ptr] as char),
                (',', 0) => {
                    let mut line = String::new();
                    io::stdin().lock().read_line(&mut line).unwrap();
                    // currently only accept text instead of raw u8 values.
                    self.mem[self.ptr] = line.chars().next().unwrap() as u8;
                }
                _ => {}
            }
            idx += 1;
        }
    }
}
