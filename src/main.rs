use std::collections::VecDeque;
use std::io::prelude::*;
use std::{fs, io};

fn main() {
    Vm::new(
        fs::read(
            fs::canonicalize(std::env::args().nth(1).unwrap_or_else(|| help()))
                .expect("could not canonicalize argument"),
        )
        .expect("could not read file"),
    )
    .interpret()
}

fn help() -> ! {
    println!(
        "usage: {} <filename>",
        std::env::current_exe().unwrap_or("tag".into()).display()
    );
    std::process::exit(-1);
}

macro_rules! unwrap {
    ($q: expr) => {
        match $q {
            Some(x) => x,
            _ => break,
        }
    };
}

struct Buf(u8, u8);

impl Buf {
    fn new(x: u8) -> Self {
        Self(x, 0)
    }

    fn push(&mut self, bit: bool) {
        self.0 &= !(1 << self.1);
        self.0 |= (bit as u8) << self.1;
        self.1 += 1;
    }

    fn pop(&mut self) -> Option<bool> {
        if self.1 == 0 {
            None
        } else {
            self.1 -= 1;
            Some((self.0 >> self.1) & 1 > 0)
        }
    }
}

#[derive(Debug)]
enum Instr {
    If,
    Skip,
    Input,
    Output,
    Push(bool),
    LoopStart,
    LoopEnd(usize),
}

fn translate(prog: Vec<u8>) -> Vec<Instr> {
    let mut translated = Vec::new();
    let mut starts = Vec::new();
    let mut prog = prog.into_iter();

    while let Some(b) = prog.next() {
        let translated_instr = match b {
            b'?' => Instr::If,
            b'#' => Instr::Skip,
            b'0' => Instr::Push(false),
            b'1' => Instr::Push(true),
            b'[' => {
                starts.push(translated.len());
                Instr::LoopStart
            }
            b']' => {
                let corr = starts.pop().expect("mismatched brackets");
                Instr::LoopEnd(corr)
            }
            b',' => Instr::Input,
            b'.' => Instr::Output,

            _ => continue,
        };

        translated.push(translated_instr);
    }

    assert!(starts.is_empty(), "mismatched brackets");

    translated
}

struct Vm {
    prog: Vec<Instr>,
    queue: VecDeque<bool>,
    bytes: io::Bytes<io::Stdin>,
    inp: Buf,
    out: Buf,
}

impl Vm {
    fn new(prog: Vec<u8>) -> Self {
        let queue = VecDeque::<bool>::new();
        let bytes = io::stdin().bytes();
        let inp = Buf::new(0);
        let out = Buf::new(0);

        Self {
            queue,
            bytes,
            inp,
            out,

            prog: translate(prog),
        }
    }

    fn interpret(&mut self) {
        let mut pc = 0;

        while pc < self.prog.len() {
            match self.prog[pc] {
                Instr::Push(x) => {
                    self.queue.push_back(x);
                }

                Instr::Input => {
                    let inp = match self.inp.pop() {
                        Some(x) => x,
                        _ => {
                            self.inp = Buf(unwrap!(self.bytes.next()).unwrap(), 8);
                            self.inp.pop().unwrap()
                        }
                    };

                    self.queue.push_back(inp);
                }

                Instr::Output => {
                    self.out.push(*unwrap!(self.queue.get(0)));

                    if self.out.1 >= 8 {
                        io::stdout()
                            .write_all(&[self.out.0.reverse_bits()])
                            .expect("failed to write to stdout");

                        self.out.0 = 0;
                        self.out.1 = 0;
                    }
                }

                Instr::LoopStart => {
                    unwrap!(self.queue.pop_front());
                }

                Instr::LoopEnd(start) => {
                    pc = start;
                }

                Instr::If => {
                    if !*unwrap!(self.queue.get(0)) {
                        pc += 1;
                    }
                }

                Instr::Skip => {
                    pc += 1;
                }
            }

            pc += 1
        }
    }
}
