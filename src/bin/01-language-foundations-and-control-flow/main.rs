#[path = "../../../examples/01-language-foundations-and-control-flow/expressions-shadowing.rs"]
mod expressions_shadowing;

#[path = "../../../examples/01-language-foundations-and-control-flow/struct-basics.rs"]
mod struct_basics;

#[path = "../../../examples/01-language-foundations-and-control-flow/enum-basics.rs"]
mod enum_basics;

#[path = "../../../examples/01-language-foundations-and-control-flow/match-basics.rs"]
mod match_basics;

#[path = "../../../examples/01-language-foundations-and-control-flow/match-guard.rs"]
mod match_guard;

#[path = "../../../examples/01-language-foundations-and-control-flow/pattern-ownership.rs"]
mod pattern_ownership;

#[path = "../../../examples/01-language-foundations-and-control-flow/control-flow-patterns.rs"]
mod control_flow_patterns;

#[path = "../../../examples/01-language-foundations-and-control-flow/structs-enums-patterns.rs"]
mod structs_enums_patterns;

fn main() {
    println!("P0: Bindings, Struct and Enum construction");
    expressions_shadowing::main();
    struct_basics::main();
    enum_basics::main();
    println!("P0: Match and guard");
    match_basics::main();
    match_guard::main();
    println!("P0: Pattern ownership and combinations");
    pattern_ownership::main();
    control_flow_patterns::main();
    structs_enums_patterns::main();
}
