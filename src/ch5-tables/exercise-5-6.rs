/*
Write a function to test whether a given table is a valid sequence.
*/

fn is_sequence(v: &Vec<i32>) -> bool {
    if v.is_empty() {
        return true;
    }

    let mut last_val = v[0];

    for i in 1..v.len() {
        if v[i] != last_val + 1 {
            return false;
        }
        last_val = v[i];
    }

    return true;
}

fn main() {
    assert!(is_sequence(&vec![]));
    assert!(is_sequence(&vec![1]));
    assert!(is_sequence(&vec![1,2,3]));
    assert!(!is_sequence(&vec![1,3,2]));
    assert!(!is_sequence(&vec![1,2,4]));
    assert!(!is_sequence(&vec![1,3,5]));
}
