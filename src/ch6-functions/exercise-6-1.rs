/*
Write a function that takes an array and prints all its elements.
*/

use std::fmt::Display;

fn print_vec<T: Display>(v: &Vec<T>) {
    for e in v {
        println!("{}", e);
    }
}

fn print_vec2<T: Display>(v: &Vec<T>) {
    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}

fn main() {
    print_vec(&vec![1,2]);
    print_vec2::<i32>(&vec![]);
    print_vec2(&vec![1,2]);
}
