/*
Q: Modify the eight-queen program so that it stops after printing
   the first solution.
*/

// This compiled without warnings, but fails at runtime
// due to index out of bounds

const N: usize = 8;

// Check whether position (n,c) is free from attacks
fn isplaceok(a: &mut Vec<usize>, n: usize, c: usize) -> bool {
    for i in 1..n-1 { // For each queen already placed
        if a[i] == c ||             // Same column?
            a[i] - i == c - n ||    // Same diagonal?
            a[i] + i == c + n {     // Same diagonal?
            return false; // Place can be attacked
        }
    }
    return true; // No attacks; place is OK
}

// Print a board
fn printsolution(a: &Vec<usize>) {
    for i in 1..N {    // for each row
        for j in 1..N { // and for each column
            // write "X" or "-" plus a space
            print!("{} ", if a[i] == j { "X" } else { "-" })
        }
        println!()
    }
    println!()
}

// Add to board a all the queens from n to N.
// Returns true if solution found, else returns false.
fn addqueen(a: &mut Vec<usize>, n: usize) -> bool {
    if n > N { // All queens have been placed?
        printsolution(a);
        return true;
    }

    // Try to place the nth queen
    for c in 1..N {
        if isplaceok(a, n, c) {
            a[n] = c; // Place the nth queen at column c
            let sol_found = addqueen(a, n + 1);
            if sol_found {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let mut a: Vec<usize> = Vec::new();
    addqueen(&mut a, 1);
}
