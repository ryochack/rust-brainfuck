use std::io;

mod brainfuck;

fn main() {
    let mut r = io::stdin();
    let mut b = brainfuck::Brainfuck::new(&mut r);
    b.interpret();
}
