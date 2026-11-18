// Exercises for lesson 1. Replace every `todo!()` with your own code, then run:
//
//     cargo test -p exercises-01
//
// Silences "unused variable" warnings while the functions are still todo!().
// Delete this line once you've finished all exercises.
#![allow(unused_variables)]

/// 1. Convert Celsius to Fahrenheit: F = C × 9/5 + 32
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

/// 2. Return true when `n` is even. Hint: the `%` operator works like in JS.
pub fn is_even(n: i32) -> bool {
    todo!()
}

/// 3. Return the largest of three numbers.
/// Use `if` as an expression. (Don't use `.max()` yet, that's too easy.)
pub fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    todo!()
}

/// 4. Return 1 + 2 + … + n using a `for` loop over a range.
/// sum_to(0) is 0.
pub fn sum_to(n: u32) -> u32 {
    todo!()
}

/// 5. Classic FizzBuzz for a single number:
/// "FizzBuzz" if divisible by 3 and 5, "Fizz" if by 3, "Buzz" if by 5,
/// otherwise the number itself as text.
/// Hint: `n.to_string()` turns a number into a String, and `"Fizz".to_string()`
/// turns a &str into a String.
pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

/// 6. Count the vowels (a, e, i, o, u; upper or lower case) in `text`.
/// Hint: `for c in text.chars()` loops over each character, and
/// `c.to_ascii_lowercase()` lowercases a char. Chars use single quotes: 'a'.
pub fn count_vowels(text: &str) -> usize {
    todo!()
}

/// 7. Bonus: the number of steps the Collatz sequence takes to reach 1.
/// While n != 1: if n is even, n = n / 2, otherwise n = 3n + 1.
/// collatz_steps(1) is 0, collatz_steps(6) is 8.
/// Hint: the parameter is immutable. Shadow it with `let mut n = n;`.
pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_fahrenheit_converts() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn is_even_works() {
        assert!(is_even(4));
        assert!(is_even(0));
        assert!(is_even(-2));
        assert!(!is_even(7));
        assert!(!is_even(-3));
    }

    #[test]
    fn max_of_three_finds_largest() {
        assert_eq!(max_of_three(1, 2, 3), 3);
        assert_eq!(max_of_three(3, 2, 1), 3);
        assert_eq!(max_of_three(2, 3, 1), 3);
        assert_eq!(max_of_three(-5, -1, -9), -1);
        assert_eq!(max_of_three(7, 7, 7), 7);
    }

    #[test]
    fn sum_to_adds_range() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_to(5), 15);
        assert_eq!(sum_to(100), 5050);
    }

    #[test]
    fn fizzbuzz_handles_all_cases() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(98), "98");
    }

    #[test]
    fn count_vowels_counts() {
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("rhythm"), 0);
        assert_eq!(count_vowels("Rust"), 1);
        assert_eq!(count_vowels("AEIOU aeiou"), 10);
        assert_eq!(count_vowels("Hello, World!"), 3);
    }

    #[test]
    fn collatz_steps_counts() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }
}
