// Multiples of 3 or 5
// Show HTML problem content 
// Problem 1
// If we list all the natural numbers below 10 that are multiples of 3 or 5, 
// we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn euler1(param: i32) -> i32 {
    let mut sum = 0;

    for n in 1..param {
        if (n % 3 == 0) || (n % 5 == 0) {
            sum += n;
            println!("{}", n)
        }
    }

    return sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn shuld_pass_euler1() {
        assert_eq!(euler1(10), 23);
        assert_eq!(euler1(1000), 233168);
    }
}