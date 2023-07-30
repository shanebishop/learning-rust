/*
Write a function that inserts all elements of a given list into a given
position of another given list.
*/

fn insert_vec(insert_into: &mut Vec<i32>, index: usize, to_insert: &Vec<i32>) {
    let old_len = to_insert.len();
    let to_insert_len = to_insert.len();

    // Reserve space for new elements.
    // Note that Vec::reserve doesn't increase the length, so assignments after
    // Vec::reserve would not work, which is why we use Vec::resize instead.
    insert_into.resize(to_insert_len, 0);

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
    assert_eq!(v, vec![1,4,5,2,3])
}
