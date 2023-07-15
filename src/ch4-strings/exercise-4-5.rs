/*
Write a function to remove a slice from a string; the slice should be given
by its initial position and its length. Assume input is ASCII-only.
*/

fn remove(s: &str, start: usize, length: usize) -> String {
    return format!("{}{}", &s[..start], &s[start+length..]);
}

fn main() {
    assert_eq!(remove("hello world", 6, 4), "hello d"); // Cuts out "worl"
}
