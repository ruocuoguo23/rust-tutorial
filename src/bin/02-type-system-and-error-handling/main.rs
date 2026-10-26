#[path = "../../../examples/02-type-system-and-error-handling/expressions-shadowing.rs"]
mod expressions_shadowing;
#[path = "../../../examples/02-type-system-and-error-handling/option-result-propagation.rs"]
mod option_result_propagation;
#[path = "../../../examples/02-type-system-and-error-handling/pattern-ownership.rs"]
mod pattern_ownership;
#[path = "../../../examples/02-type-system-and-error-handling/structs-enums-patterns.rs"]
mod structs_enums_patterns;
#[path = "../../../examples/02-type-system-and-error-handling/typed-errors.rs"]
mod typed_errors;

fn main() -> Result<(), typed_errors::PortError> {
    println!("P0: Expressions / Struct / Enum / Pattern Matching");
    expressions_shadowing::main();
    structs_enums_patterns::main();
    pattern_ownership::main();
    println!("P0: Option / Result / ?");
    option_result_propagation::main()?;
    println!("P1: Typed errors and their source");
    typed_errors::main();
    Ok(())
}
