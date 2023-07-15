/*
Write a function to insert a string into a given position of another one.
Assume both input strings contain only ASCII.
*/

// This could also be done with String::insert_str
fn insert(s: &str, index: usize, to_insert: &str) -> String {
    return format!("{}{}{}", &s[..index], to_insert, &s[index..]);
}

fn main() {
    // Test for strs
    assert_eq!(insert("hello world", 0, "start: "), "start: hello world");
    assert_eq!(insert("hello world", 6, "small "), "hello small world");

    // Test for Strings
    assert_eq!(insert(&String::from("hello world"), 0, "start: "), "start: hello world");
    assert_eq!(insert(&String::from("hello world"), 6, "small "), "hello small world");
}
