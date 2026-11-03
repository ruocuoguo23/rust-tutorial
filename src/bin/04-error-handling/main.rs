#[path = "../../../examples/04-error-handling/option-basics.rs"]
mod option_basics;

#[path = "../../../examples/04-error-handling/result-basics.rs"]
mod result_basics;

#[path = "../../../examples/04-error-handling/result-explicit-return.rs"]
mod result_explicit_return;

#[path = "../../../examples/04-error-handling/result-question-mark.rs"]
mod result_question_mark;

#[path = "../../../examples/04-error-handling/option-question-mark.rs"]
mod option_question_mark;

#[path = "../../../examples/04-error-handling/result-option-states.rs"]
mod result_option_states;

#[path = "../../../examples/04-error-handling/option-result-propagation.rs"]
mod option_result_propagation;

#[path = "../../../examples/04-error-handling/typed-errors.rs"]
mod typed_errors;

fn main() -> Result<(), typed_errors::PortError> {
    println!("P0: Option and Result branches");
    option_basics::main();
    result_basics::main();
    println!("P0: Explicit return, Result ? and Option ?");
    result_explicit_return::main();
    result_question_mark::main();
    option_question_mark::main();
    println!("P0: Three states and configuration combination");
    result_option_states::main();
    option_result_propagation::main()?;
    println!("P1: Typed errors and policy boundaries");
    typed_errors::main();
    Ok(())
}
