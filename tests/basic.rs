// © 2014-2025 Ultralytics Inc. 🚀 All rights reserved. CONFIDENTIAL: Unauthorized use or distribution prohibited.

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
