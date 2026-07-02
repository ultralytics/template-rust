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

#[test]
fn cli_prints_example_output() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ultralytics-template-rust"))
        .output()
        .expect("failed to run example binary");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Added 1 + 2 = 3\n"
    );
    assert!(output.stderr.is_empty());
}
