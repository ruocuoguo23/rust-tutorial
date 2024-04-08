// This is the main function
fn main() {
    // Immutable references
    let s1 = String::from("hello");
    // Create two immutable references to s1
    let _r1 = &s1;
    let _r2 = &s1;
    // This is fine, multiple immutable references are allowed
    println!("{} and {}", _r1, _r2);

    // Mutable references
    let mut s2 = String::from("hello");
    // Create a mutable reference to s2
    let r3 = &mut s2;
    // This is fine, we can use r3 to change s2
    r3.push_str(", world");
    println!("{}", r3);
    // r3's scope ends here, so we can create a new mutable reference after

    // This will not compile because you cannot have a mutable reference
    // while immutable references exist
    /*
    let mut s3 = String::from("hello");
    let r4 = &s3; // No problem
    let r5 = &s3; // No problem
    let r6 = &mut s3; // BIG problem
    println!("{}, {}, and {}", r4, r5, r6);
    */

    // This works, r1 and r2 are no longer used after this point
    let mut s4 = String::from("hello");
    {
        let _r7 = &s4; // Immutable borrow starts here
        let _r8 = &s4; // Immutable borrow starts here
    } // Immutable borrows end here
    let _r9 = &mut s4; // Mutable borrow is allowed after immutable borrows end

    // NLL example: non-lexical lifetimes
    // This will compile because the scope of the immutable references r10 and r11
    // ends before the mutable reference r12 is created
    let mut s5 = String::from("hello");
    let r10 = &s5; // No problem
    let r11 = &s5; // No problem
    println!("{} and {}", r10, r11); // r10 and r11 are no longer used after this line
    let r12 = &mut s5; // No problem, because r10 and r11 are not used after this
    println!("{}", r12);

    // Dangling references: This will not compile in Rust
    // let reference_to_nothing = dangle();
}

// This function attempts to create a dangling reference, which Rust prevents at compile time
/*
fn dangle() -> &String { // This function returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // We attempt to return a reference to s
} // Here, s goes out of scope and is dropped. Its memory goes away.
  // Danger!
*/

// This function does not create a dangling reference
fn no_dangle() -> String {
    let s = String::from("hello");
    s // s is returned and moves out to the calling function
}