/*
We can represent a polynomial a_{n}x^{n} + ... + a_{1}x^{1} + a_0 in Lua as a list
of its coefficients, such as {a_{0}, ..., a_{n}}.
Write a function that takes a polynomial (represented as a table) and a value
for x and returns the polynomial value.
*/

use std::convert::TryInto;

fn polynomial_value(poly: Vec<i32>, x: i32) -> i32 {
    let mut sum = 0;

    for power in 0..poly.len() {
        sum += poly[power] * x.pow(power.try_into().unwrap());
    }

    return sum;
}

fn main() {
    // 2 + x, where x is 1
    assert_eq!(polynomial_value(vec![1, 2], 1), 3);
    // 3 + 4x + 2x^2 where x is 3
    // 3 + 12 + 18 = 33
    assert_eq!(polynomial_value(vec![3, 4, 2], 3), 33);
    // 3, where x is 2
    assert_eq!(polynomial_value(vec![3], 2), 3);
}
