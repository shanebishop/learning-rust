/*
Write a program that reads a text file and outputs it with its lines sorted
in alphabetical order. When called with no arguments, it should read from
standard input and write to standard output. When called with one file-name
argument, it should read from that file and write to standard output. When
called with two file-name arguments, it should read from the first file and
write to the second.
*/

use std::env;
use std::process::exit;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (reader, path) =
        if args.len() > 1 {
            let path = &args[1];

            let f = match File::open(path) {
                Err(why) => die(format!("Failed to open input file '{}': {}", path, why)),
                Ok(file) => file,
            };

            let reader: Box<dyn Read> = Box::new(f);

            (reader, path.clone())
        } else {
            let reader: Box<dyn Read> = Box::new(std::io::stdin());
            (reader, String::from("<stdin>"))
        };

    let mut lines = Vec::<String>::new();
    for res in BufReader::new(reader).lines() {
        match res {
            Err(why) => die(format!("Failed to read all lines from '{}': {}", path, why)),
            Ok(line) => lines.push(line),
        };
    }

    lines.sort();

    for line in lines {
        println!("{}", line);
    }

    exit(0);
}

fn die(s: String) -> ! {
    eprintln!("{}", s);
    exit(1);
}
