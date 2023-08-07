/*
Write a program that reads a text file and rewrites it with its lines sorted
in alphabetical order. When called with no arguments, it should read from
standard input and write to standard output. When called with one file-name
argument, it should read from that file and write to standard output. When
called with two file-name arguments, it should read from the first file and
write to the second.

NOTE: This does not handle read or write failures.
*/

use std::env;
// use std::io::stdin;
// use std::os::fd::OwnedFd;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        eprintln!("Error: Missing input file argument.");
        exit(1);
    }

    let f = File::open(&args[0])?;

    let mut lines = Vec::<String>::new();
    for res in BufReader::new(f).lines() {
        lines.push(res.expect("Failed to read all lines from input file"));
    }

    lines.sort();

    for line in lines {
        println!(line);
    }

    exit(0);
}
