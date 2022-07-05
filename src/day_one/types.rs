/// There are various types in Rust
// First and foremost we'll be addresssing primitives
// These are boolean, number(numeric), textual

pub fn exec_types() {
    let x: bool = false;
    if !x {
        println!("It is not true")
    }

    // These are the examples of unsigned integers of size 8, 16, 32, 64 and 128
    // The max values of these types take the format of 2^n - 1
    // Example: u8 will take 2^8-1 that is 0-255
    // Maximum value the humungous number can take is 2^128 - 1

    let small_number: u8 = 0;
    let smaller_number: u16 = 0;
    let somewhat_bigger_number: u32 = 0;
    let a_bigger_number: u64 = 0;
    let humungous_number: u128 = 0;

    println!(
        "The numbers are in the following order\n:\n {}, {}, {}, {}, {}",
        small_number, smaller_number, somewhat_bigger_number, a_bigger_number, humungous_number
    );

    // To deal with the signed integers that is negative numbers, you can use the i-type.
    // These numbers will take ranges from -2^n-1 to 2^n-1

    let signed_small: i8 = -1;
    let signed_somewhat_bigger: i16 = -1;
    let signed_big_number: i32 = -1;
    let signed_bigger_number: i64 = -1;
    let signed_humungous: i128 = -1;

    println!(
        "The signed numebrs are in the following order of size taken by their respective types.\n:\n {}, {}, {}, {}, {}",
        signed_small, signed_somewhat_bigger,signed_big_number, signed_bigger_number, signed_humungous
    );

    // Character and Text types
    let new_char: char = 'a'; // Remember, using a double quote is not allowed.
    let new_string: &str = "Hello World";

    // If you're from a python/TS/JS background, chars are basically smaller versoins of string, they allow you to store single characters

    println!("The characters/texts are {}, {}", new_char, new_string);

    // Let us try to do this from a reference perspective

    let new_reference_char: char = char::from('h');
    let new_reference_string: String = String::from("Hello!");

    println!("The characters/texts are {}, {}", new_reference_char, new_reference_string);

    // Now you might be wondering what is the difference between string and string-from combo
    // One is an actual reference where as the second one is allotted during the run-time that is post compilation.
    // It is an inexpensive conversion, one can convert from &Str to string using to_string.

}
