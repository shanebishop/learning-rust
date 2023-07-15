/*
Q: Write a simple script that prints its own name without
   knowing it in advance.

The name of the lua script being executed is always stored in
arg[0] (arg is a predefined global variable). For more details on
program arguments, see page 10 of the book.
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[0]);
}
