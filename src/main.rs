struct NoClone {
    value: i32,
}

fn process_value(no_clone: NoClone) -> Option<NoClone> {
    // Processing succeeds if the value is divisible by 5
    if no_clone.value % 5 == 0 {
        None  // Success, consume the value
    } else {
        Some(NoClone { value: no_clone.value + 1 })  // Failure, increment and return the value for a retry
    }
}

fn main() {
    let mut retry_option: Option<NoClone> = Some(NoClone { value: 11 }); // Start with a non-clonable value

    while let Some(no_clone) = retry_option.take() { // Take the value out of the option
        println!("Processing value: {}", no_clone.value);
        retry_option = process_value(no_clone); // Process and potentially get a new option

        if retry_option.is_some() {
            println!("Failed to process, retrying ...");
        }
    }

    println!("Finished processing with success");
}
