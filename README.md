# learning-rust

This repository contains my solutions to the exercises provided in the book
[*Programming in Lua, Fourth Edition* by Roberto
Ierusalimschy](https://www.lua.org/pil/#4ed), but for Rust.

## Building

As each exercise is a standalone program, no `Cargo.toml` is used. Instead,
each exercise can be compiled and run separately. For example:
```bash
rustc src/ch1-getting-started/exercise-1-8.rs
./exercise-1-8 # Or .\exercise-1-8.exe on Windows
```
