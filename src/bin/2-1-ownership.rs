fn main() {
    // Basic types implement the Copy trait, so the original variable can be used after assignment.
    let x = 5;
    let y = x; // The value of `x` is copied to `y`, `x` is still valid.
    println!("x = {}, y = {}", x, y);

    // String type is stored on the heap, so assignment involves ownership transfer.
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of `s1` is transferred to `s2`, `s1` is no longer valid.
    // println!("s1 = {}", s1); // This line will cause a compile error because `s1` is no longer valid.
    println!("s2 = {}", s2);

    // Use .clone() to create a deep copy of the data on the heap.
    let s3 = s2.clone(); // The data of `s2` is deeply copied to `s3`, `s2` is still valid.
    println!("s2 = {}, s3 = {}", s2, s3);

    // Function parameters and return values also involve ownership.
    let s4 = takes_and_gives_back(s3); // Ownership of `s3` is transferred to the function, then returned to `s4`.
    // println!("s3 = {}", s3); // This line will cause a compile error because the ownership of `s3` has been transferred.
    println!("s4 = {}", s4);
}

// This function takes ownership of a string and then returns it.
fn takes_and_gives_back(a_string: String) -> String { // `a_string` comes into scope.
    println!("Inside the function: {}", a_string);
    a_string // Returns `a_string`, ownership is transferred back to the caller.
}
