#[derive(Debug)]
struct ProcessingResult {
    steps_completed: usize,
    final_value: i32,
    modifications_made: usize,
}

fn main() {
    let mut blockchain_data = vec![100, 250, 75, 300, 150];

    println!("=== Advanced Reference Chain ===");
    println!("Initial blockchain data: {:?}", blockchain_data);

    // Complex processing chain
    match process_blockchain_data(&mut blockchain_data) {
        Ok(result) => {
            println!("Processing successful: {:?}", result);
        },
        Err(e) => {
            println!("Processing failed: {}", e);
        }
    }

    println!("Final blockchain data: {:?}", blockchain_data);

    // Test reference sharing patterns
    test_reference_sharing(&blockchain_data);
}

fn process_blockchain_data(data: &mut Vec<i32>) -> Result<ProcessingResult, String> {
    // TODO: Implement a complex processing chain that:
    // 1. Validates data (immutable reference)
    // 2. Processes transactions (mutable reference)
    // 3. Calculates metrics (immutable reference)
    // 4. Applies final adjustments (mutable reference)

    let mut result = ProcessingResult {
        steps_completed: 0,
        final_value: 0,
        modifications_made: 0,
    };

    // Step 1: Validation
    validate_data(data)?;
    result.steps_completed += 1;

    // Step 2: Processing (this should modify data)
    let mods = process_transactions(data)?;
    result.modifications_made = mods;
    result.steps_completed += 1;

    // Step 3: Analysis (read-only)
    result.final_value = calculate_final_value(data)?;
    result.steps_completed += 1;

    // Step 4: Final adjustments
    apply_final_adjustments(data)?;
    result.steps_completed += 1;

    Ok(result)
}

fn validate_data(data: &Vec<i32>) -> Result<(), String> {
    // TODO: Validate that data meets requirements
    // - Not empty
    if data.len() > 0 {
        println!("Data NOT empty: {:?}", data );
    }

    else {
        println!("Data is EMPTY: {:?}", data );
    }


    // - All values positive

    let positive_values: Vec<i32> = data
        .iter()       // iterate over &i32
        .copied()     // turn &i32 into i32
        .filter(|v| *v > 0)  // keep only positive numbers
        .collect();   // collect into a Vec<i32>


    println!("Positive values: {:?}", positive_values);


    // - Within reasonable ranges
    Ok(())
}

fn process_transactions(data: &mut Vec<i32>) -> Result<usize, String> {
    // TODO: Process and modify the transaction data
    // Return number of modifications made
    Ok(0)
}

fn calculate_final_value(data: &Vec<i32>) -> Result<i32, String> {
    // TODO: Calculate some final metric from the data
    Ok(0)
}

fn apply_final_adjustments(data: &mut Vec<i32>) -> Result<(), String> {
    // TODO: Apply any final adjustments to the data
    Ok(())
}

fn test_reference_sharing(data: &Vec<i32>) {
    println!("\n=== Reference Sharing Test ===");

    // Multiple immutable references are allowed
    let ref1 = data;
    let ref2 = data;
    let ref3 = data;

    // All can be used simultaneously
    println!("Ref1 length: {}", ref1.len());
    println!("Ref2 sum: {}", ref2.iter().sum::<i32>());
    println!("Ref3 max: {:?}", ref3.iter().max());

    // Test function calls with multiple references
    compare_references(ref1, ref2);
}

fn compare_references(data1: &Vec<i32>, data2: &Vec<i32>) {
    // TODO: Compare two references to the same data
    // This demonstrates that multiple immutable references work
    println!("References point to same data: {}",
             std::ptr::eq(data1, data2));
}

// Advanced: Function that returns references
fn find_largest_element(data: &Vec<i32>) -> Option<&i32> {
    // TODO: Return a reference to the largest element
    // This demonstrates returning borrowed data
    None
}

fn find_elements_above_threshold(data: &Vec<i32>, threshold: i32) -> Vec<&i32> {
    // TODO: Return references to elements above threshold
    // This creates a vector of references, not owned values
    vec![]
}
