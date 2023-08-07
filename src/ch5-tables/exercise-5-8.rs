/*
The Lua table library offers a function table.concat, which receives a list of
strings and returns their concatenation:

    print(table.concat({"hello", " ", "world"}))    --> hello world

Write an equivalent of this function in Rust.
*/

fn str_concat(list: &Vec<String>) -> String {
    let mut size = 0;
    for s in list {
        size += s.len();
    }

    let mut buffer = String::with_capacity(size);
    for s in list {
        buffer.push_str(&s);
    }

    return buffer;
}

fn main() {
    assert_eq!(str_concat(&vec![String::from("hello"), String::from(" "), String::from("world")]), String::from("hello world"));
    assert_eq!(str_concat(&vec![]), String::from(""));
    assert_eq!(str_concat(&vec![String::from("foo")]), String::from("foo"));
}
