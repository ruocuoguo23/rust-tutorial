fn main() {
    // Immutable string slice
    let hello = "Hello, world!"; // This is a string slice (&str) and is immutable
    println!("{}", hello);

    // Creating a `String` from a string slice
    let mut hello_string = String::from(hello); // Convert &str to String
    println!("{}", hello_string);

    // Appending a string slice to a `String`
    hello_string.push_str(" Welcome to Rust!"); // Append a string slice to a String
    println!("{}", hello_string);

    // Attempting to index into a String - this will not compile in Rust
    // let first_char = hello_string[0];
    // println!("First character: {}", first_char);

    // Instead, we can use chars() to access individual Unicode scalar values
    if let Some(first_char) = hello_string.chars().nth(0) { // Get first character
        println!("First character: {}", first_char);
    }

    // Slicing a `String` - needs to be on character boundaries
    let hello_slice = &hello_string[0..5]; // Correct slicing at character boundaries
    println!("Slice: {}", hello_slice);

    // If we try to slice at non-character boundary, it will cause a runtime error
    // let broken_slice = &hello_string[0..4]; // This will panic at runtime

    // Concatenating strings with the `+` operator
    let world_string = String::from("Hello, Rustaceans!");
    let exclamation = "!";
    let combined = world_string + exclamation; // Note: `world_string` is moved here and can no longer be used
    println!("Combined: {}", combined);

    // Using format! macro for concatenation without taking ownership
    let part_one = "How";
    let part_two = "are";
    let part_three = "you?";
    let combined_with_format = format!("{} {} {}", part_one, part_two, part_three); // No ownership taken
    println!("Combined with format!: {}", combined_with_format);

    // Escaping characters
    let escaped_string = "This string contains a unicode character: \u{1F600}";
    println!("Escaped string: {}", escaped_string);

    // Raw strings
    let raw_string = r#"This is a raw string with "quotes" and \t no escape characters."#;
    println!("Raw string: {}", raw_string);

    // UTF-8 iteration over characters
    let unicode_string = "नमस्ते";
    for c in unicode_string.chars() {
        println!("{}", c);
    }

    // UTF-8 iteration over bytes
    for b in unicode_string.bytes() {
        println!("{}", b);
    }

    // Attempting to create a string slice from String using range indexing
    // This is safe because the character boundaries are respected
    let hello_world = String::from("Hello, 世界!");
    let hello = &hello_world[0..7]; // "Hello, " is 7 bytes
    println!("Hello slice: {}", hello);
    let world = &hello_world[7..]; // Rest of the string, "世界!" is correctly sliced
    println!("World slice: {}", world);
}
