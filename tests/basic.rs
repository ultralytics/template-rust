// Ultralytics 🚀 AGPL-3.0 License - https://ultralytics.com/license

use ultralytics_template_rust::{add_numbers, run_example};

#[test]
fn add_numbers_handles_signed_values() {
    assert_eq!(add_numbers(2, 3), 5);
    assert_eq!(add_numbers(-1, 1), 0);
    assert_eq!(add_numbers(-1, -1), -2);
}

#[test]
fn example_output_matches_cli() {
    assert_eq!(run_example(), "Added 1 + 2 = 3");
}
