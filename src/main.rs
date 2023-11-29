struct NonCopyableValue {
    value: i32,
}

fn process_value(value: NonCopyableValue) -> Option<NonCopyableValue> {
    // Processing succeeds if the value is divisible by 5
    if value.value % 5 == 0 {
        None  // Success, consume the value
    } else {
        Some(NonCopyableValue { value: value.value + 1 })  // Failure, increment and return the value for a retry
    }
}

fn main() {
    let mut retry_option: Option<NonCopyableValue> = Some(NonCopyableValue { value: 11 }); // Start with a non-copyable value

    while let Some(value) = retry_option.take() { // Take the value out of the option
        println!("Processing value: {}", value.value);
        retry_option = process_value(value); // Process and potentially get a new option

        if retry_option.is_some() {
            println!("Failed to process, retrying with incremented value...");
        }
    }

    println!("Finished processing with success");
}
