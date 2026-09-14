mod errors;
mod patterns;

fn main() -> Result<(), errors::PortError> {
    println!("P0: Expressions / Struct / Enum / Pattern Matching");
    patterns::run();
    println!("P0: Option / Result / ?");
    errors::run()?;
    println!("P1: Typed errors and their source");
    errors::error_design();
    Ok(())
}
