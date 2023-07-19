use std::io;
use std::io::{BufWriter, ErrorKind::BrokenPipe, StdoutLock, Write};

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut stdout = BufWriter::new(io::stdout().lock());
    for line in io::stdin().lines() {
        print_words(&line.unwrap(), &mut stdout);
    }
}

fn print_words(line: &String, out: &mut BufWriter<StdoutLock>) {
    for word in line.unicode_words() {
        match writeln!(out, "{word}") {
            Err(e) if e.kind() == BrokenPipe => { std::process::exit(0) }
            Err(_) => panic!("Unexpected error trying to write word"),
            _ => break
        }
    }
}
