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
        std::env::current_exe().unwrap_or("ctfuck".into()).display()
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
    fn new() -> Self {
        Self(0, 0)
    }

    fn push(&mut self, bit: bool) {
        self.0 &= !(1 << self.1);
        self.0 |= (bit as u8) << self.1;
        self.1 += 1;
    }

    fn pop(&mut self) -> Option<bool> {
        if self.1 == 8 {
            None
        } else {
            let res = Some((self.0 >> self.1) & 1 > 0);
            self.1 += 1;
            res
        }
    }
}

#[derive(Debug)]
enum Instr {
    Pop,
    Input,
    Output,
    Push(bool),
    Jmp(usize, usize),
}

fn translate(prog: Vec<u8>) -> Vec<Instr> {
    let mut translated = Vec::new();
    let mut prog = prog.into_iter();
    let mut loc = vec![0];

    while let Some(b) = prog.next() {
        let translated_instr = match b {
            b'\n' => {
                loc.push(translated.len());
                continue;
            }
            b'$' => Instr::Pop,
            b'0' => Instr::Push(false),
            b'1' => Instr::Push(true),
            b'[' => parse_cond(&mut prog),
            b',' => Instr::Input,
            b'.' => Instr::Output,

            _ => continue,
        };

        translated.push(translated_instr);
    }

    for (i, x) in translated.iter_mut().enumerate() {
        if let Instr::Jmp(x, y) = x {
            let f = |x: &mut usize| *x = if *x == 0 { i + 1 } else { loc[*x - 1] };

            f(x);
            f(y);
        }
    }

    translated
}

fn parse_cond(x: &mut impl Iterator<Item = u8>) -> Instr {
    let if_ = parse_int(x, b'|');
    let else_ = parse_int(x, b']');
    Instr::Jmp(if_, else_)
}

fn parse_int(x: &mut impl Iterator<Item = u8>, end: u8) -> usize {
    let mut res = 0;

    loop {
        let c = x
            .next()
            .unwrap_or_else(|| panic!("expected {} or number, found EOF", end as char))
            .to_ascii_lowercase();

        if c == end {
            break;
        }

        res = res * 10 + parse_byte(c);
    }

    res
}

fn parse_byte(c: u8) -> usize {
    (match c {
        b'0'..=b'9' => c - b'0',
        _ => panic!("unexpected character during conditional jump parsing"),
    })
    .into()
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
        let inp = Buf(0, 8);
        let out = Buf::new();

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
                            self.inp = Buf(self.bytes.next().unwrap_or(Ok(0)).unwrap(), 0);
                            self.inp.pop().unwrap()
                        }
                    };

                    self.queue.push_back(inp);
                }

                Instr::Output => {
                    self.out.push(*unwrap!(self.queue.get(0)));

                    if self.out.1 >= 8 {
                        let mut stdout = io::stdout();

                        stdout
                            .write_all(&[self.out.0])
                            .expect("failed to write to stdout");
                        stdout.flush().unwrap();

                        self.out = Buf::new();
                    }
                }

                Instr::Jmp(if_, else_) => {
                    if *unwrap!(self.queue.get(0)) {
                        pc = if_;
                    } else {
                        pc = else_;
                    }

                    continue;
                }

                Instr::Pop => {
                    unwrap!(self.queue.pop_front());
                }
            }

            pc += 1
        }

        if self.out.1 > 0 {
            io::stdout()
                .write_all(&[self.out.0])
                .expect("failed to write to stdout");
        }
    }
}
