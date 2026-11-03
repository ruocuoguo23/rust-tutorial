#[path = "../../../examples/09-modules-cargo-macros-and-testing/limit-demo/src/config.rs"]
mod config;
#[path = "../../../examples/09-modules-cargo-macros-and-testing/macro-evaluation.rs"]
mod macro_evaluation;

fn main() {
    assert_eq!(config::parse_limit(" 42 ").unwrap(), 42);
    println!("Module call: parsed limit = 42");
    macro_evaluation::main();
}
