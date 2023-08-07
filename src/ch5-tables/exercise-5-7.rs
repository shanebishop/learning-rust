/*
Write a function that inserts all elements of a given list into a given
position of another given list.
*/

use std::default::Default;

fn insert_vec<T>(insert_into: &mut Vec<T>, index: usize, to_insert: &Vec<T>)
    where T: Default + Copy
{
    let old_len = insert_into.len();

    if index > old_len {
        panic!("index out of bounds");
    }

    let to_insert_len = to_insert.len();

    // Reserve space for new elements.
    // Note that Vec::reserve doesn't increase the length, so assignments after
    // Vec::reserve would not work. Vec::resize also does not work if we want to
    // use generic code, since Vec::resize requires a value to fill new elements
    // with, which is why we use Vec::resize_with instead.
    insert_into.resize_with(old_len + to_insert_len, Default::default);

    // Shift elements in insert_into down
    for i in index..old_len {
        insert_into[i+to_insert_len] = insert_into[i];
    }

    // Copy elements into place
    for i in 0..to_insert.len() {
        insert_into[i+index] = to_insert[i];
    }
}

fn main() {
    let mut v = vec![1,2,3];
    insert_vec(&mut v, 1, &vec![4,5]);
    assert_eq!(v, vec![1,4,5,2,3]);

    let mut v: Vec<i32> = vec![];
    insert_vec(&mut v, 0, &vec![]);
    assert_eq!(v, vec![]);

    let mut v: Vec<i32> = vec![];
    insert_vec(&mut v, 0, &vec![1,2,3]);
    assert_eq!(v, vec![1,2,3]);

    let mut v = vec![1,2,3];
    insert_vec(&mut v, 0, &vec![]);
    assert_eq!(v, vec![1,2,3]);
}
