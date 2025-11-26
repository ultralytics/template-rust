// Ultralytics 🚀 AGPL-3.0 License - https://ultralytics.com/license

use std::ops::Add;

/// Adds two values together.
///
/// The function works with any type that supports the `+` operator and is `Copy`, making it usable
/// with common numeric primitives such as integers or floating-point numbers.
///
/// # Examples
/// ```
/// use ultralytics_template_rust::add_numbers;
///
/// let sum = add_numbers(1_i32, 2_i32);
/// assert_eq!(sum, 3);
///
/// let float_sum = add_numbers(1.5_f64, 2.0_f64);
/// assert!((float_sum - 3.5_f64).abs() < f64::EPSILON);
/// ```
pub fn add_numbers<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Copy,
{
    a + b
}

/// Generates a human-readable summary of the example addition.
///
/// This mirrors the simple entry point showcased by the CLI in `src/main.rs` and is exposed so
/// integration tests can validate user-visible behavior without shelling out.
pub fn run_example() -> String {
    let a = 1;
    let b = 2;
    let y = add_numbers(a, b);
    format!("Added {a} + {b} = {y}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_integers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(-1, 1), 0);
        assert_eq!(add_numbers(-1, -1), -2);
    }

    #[test]
    fn adds_floats() {
        let result = add_numbers(1.5_f32, 2.25_f32);
        assert!((result - 3.75_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn produces_example_output() {
        let output = run_example();
        assert_eq!(output, "Added 1 + 2 = 3");
    }
}
