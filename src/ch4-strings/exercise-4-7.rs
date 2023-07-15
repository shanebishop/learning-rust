/*
Write a function to check whether a given string is a palindrome.
*/

// This is the most readable solution, but it may not be the most efficient
// solution
fn ispali(s: &str) -> bool {
    for i in 0..s.len()/2 {
        if s.as_bytes()[i] != s.as_bytes()[s.len()-i-1] {
            return false;
        }
    }
    return true;
}

fn main() {
    assert!(ispali("step on no pets"));
    assert!(!ispali("banana"));
    assert!(ispali("racecar"));
    assert!(ispali(""));
    assert!(ispali("b"));
    assert!(ispali("bb"));
    assert!(ispali("bcb"));
    assert!(!ispali("bc"));
    assert!(!ispali("bcc"));
}
