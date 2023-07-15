fn concatenate_strings(str_1: &str, str_2: &str) -> String {
    // Creates a new mutable string named result.
    let mut result: String = String::new();
    // Pushes the contents of the first string to the result
    result.push_str(str_1);

    // Pushes the contents of the second string to the result
    result.push_str(str_2);

    // Returns the result string
    result
}

fn main() {
    // Initialize the string1 with the value "Hello, "
    let string1: String = String::from("Hello, ");

    // Initialize the string2 with the value "World, "
    let string2: String = String::from("World");

    // Call the concatenate_strings function with string1 and string2. Store the return value in the concatenated_string variable.
    let concatenated_string: String = concatenate_strings(&string1, &string2);

    // Print the concatenated_string variable to the console.
    println!("{}", concatenated_string);
}

