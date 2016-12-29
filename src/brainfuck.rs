use std::io::{self, Read, Write};
use std::{thread, time};

pub struct Brainfuck<'a> {
    code: Vec<u8>,
    code_i: usize,
    tape: Vec<u8>,
    tape_i: usize,
    loopstack: Vec<(usize, usize)>,
    reader: &'a mut Read,
}

impl<'a> Brainfuck<'a> {
    pub fn new(reader: &'a mut Read) -> Brainfuck<'a> {
        const CODE_INIT_LENGTH: usize = 256;
        const TAPE_INIT_LENGTH: usize = 256;
        const LOOPSTACK_INIT_LENGTH: usize = 4;
        let mut b = Brainfuck {
            code: Vec::with_capacity(CODE_INIT_LENGTH),
            code_i: 0,
            tape: Vec::with_capacity(TAPE_INIT_LENGTH),
            tape_i: 0,
            loopstack: Vec::with_capacity(LOOPSTACK_INIT_LENGTH),
            reader: reader,
        };
        b.tape.push(0);
        b
    }

    fn read(&mut self) -> io::Result<usize> {
        let mut buf = String::new();
        let mut read_size = 0;
        loop {
            match self.reader.read_to_string(&mut buf) {
                Ok(s) => {
                    if s > 0 {
                        // have better use
                        // `x.split_whitespace()`
                        // than
                        // `x.trim_matches(char::is_whitespace)`
                        let m = buf.split_whitespace().fold(String::new(), |base, a| base + a);
                        let bytes = m.as_bytes();
                        for i in 0..bytes.len() {
                            match bytes[i] {
                                b'[' => self.loopstack.push((self.code.len() + i, 0)),
                                b']' => {
                                    if self.loopstack.len() == 0 {
                                        panic!("not exist '['");
                                    }
                                    let mut is_found = false;
                                    for lp in self.loopstack.iter_mut().rev() {
                                        if lp.1 == 0 {
                                            // found corresponding '['
                                            lp.1 = self.code.len() + i;
                                            is_found = true;
                                            break;
                                        }
                                    }
                                    if is_found == false {
                                        panic!("not found '['");
                                    }
                                }
                                _ => (),  // do nothing
                            }
                        }
                        self.code.extend_from_slice(bytes);
                        read_size += m.len();
                        if self.loopstack.iter().filter(|x| x.1 == 0).count() > 0 {
                            continue;
                        }
                    }
                    return Ok(read_size);
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn parse(&mut self) {
        match self.code.get(self.code_i) {
            Some(&b'>') => {
                // increment pointer
                self.tape_i += 1;
                if self.tape.len() == self.tape_i {
                    self.tape.push(0);
                }
            }
            Some(&b'<') => {
                // decrement pointer
                if self.tape_i == 0 {
                    panic!("tape index underflow");
                }
                self.tape_i -= 1;
            }
            Some(&b'+') => {
                // increment value
                if let Some(e) = self.tape.get_mut(self.tape_i) {
                    *e += 1;
                }
            }
            Some(&b'-') => {
                // decrement value
                if let Some(e) = self.tape.get_mut(self.tape_i) {
                    *e -= 1;
                }
            }
            Some(&b'.') => {
                // output value
                if let Some(e) = self.tape.get(self.tape_i) {
                    print!("{}", *e as char);
                }
            }
            Some(&b',') => {
                // input value (TODO)
            }
            Some(&b'[') => {
                // jump
                if *self.tape.get(self.tape_i).unwrap() == 0 {
                    let temp_i: usize;
                    {
                        let mut filtered = self.loopstack
                                               .iter()
                                               .filter(|&x| x.0 == self.code_i);
                        let scope = filtered.next().unwrap();
                        temp_i = scope.1;
                    }
                    self.code_i = temp_i;
                }
            }
            Some(&b']') => {
                // jump
                if *self.tape.get(self.tape_i).unwrap() != 0 {
                    let temp_i: usize;
                    {
                        let mut filtered = self.loopstack
                                               .iter()
                                               .filter(|&x| x.1 == self.code_i);
                        let scope = filtered.next().unwrap();
                        temp_i = scope.0;
                    }
                    self.code_i = temp_i;
                }
            }
            Some(&c) => panic!("[{}] is unknonw", c as char),
            None => panic!("parse error!"),
        }
        self.code_i += 1;
    }

    pub fn interpret(&mut self) {
        loop {
            thread::sleep(time::Duration::from_millis(5));

            if self.code.len() == self.code_i {
                match self.read() {
                    Ok(s) => {
                        if s == 0 {
                            continue;
                        }
                    }
                    Err(_) => {
                        println!("[read error!]");
                        return;
                    }
                }
            }

            self.parse();
            let _ = io::stdout().flush();
        }
    }
}
