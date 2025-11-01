fn main() {
    println!("=== Reference Chain Demo ===");

    let mut data = vec![1, 2, 3, 4, 5];

    println!("Original data: {:?}", data);

    // Chain of immutable references
    let result = level_one(&data);
    println!("Chain result: {}", result);

    // Chain of mutable references
    level_one_mut(&mut data);
    println!("After mutable chain: {:?}", data);

    // Original data should still be accessible
    println!("Final data: {:?}", data);
}

// TODO: Implement this chain of functions
// level_one -> level_two -> level_three -> level_four

fn level_one(data: &Vec<i32>) -> i32 {
    println!("Level 1: Processing {} elements", data.len());
    level_two(data)
}

fn level_two(data: &Vec<i32>) -> i32 {
    println!("Level 2: Finding patterns in data");
    level_three(data)
}

fn level_three(data: &Vec<i32>) -> i32 {
    println!("Level 3: Performing calculations");
    level_four(data)
}

fn level_four(data: &Vec<i32>) -> i32 {
    println!("Level 4: Final computation");
    // TODO: Return some computed value from the data
    // Maybe sum of all elements, or product, or some other calculation
    let mut result: i32 = 0;

    data.iter().for_each(|x| result += x);
    println!("Level 4 - solution {}: Final computation", result);

    result
}

// Mutable reference chain
fn level_one_mut(data: &mut Vec<i32>) {
    println!("Mutable Level 1: Preparing data");
    level_two_mut(data);
}

fn level_two_mut(data: &mut Vec<i32>) {
    println!("Mutable Level 2: Transforming data");
    level_three_mut(data);
}

fn level_three_mut(data: &mut Vec<i32>) {
    println!("Mutable Level 3: Applying changes");
    level_four_mut(data);
}

fn level_four_mut(data: &mut Vec<i32>) {
    println!("Mutable Level 4: Final modifications");
    // TODO: Modify the data in some way
    // Maybe double all values, or add 1 to each, or reverse

    for x in data.iter_mut() {
        *x *= 10    ;
    }

    println!("Modified data: {:?}", data);



}

// Bonus: Mixed reference chain
fn mixed_chain_start(data: &mut Vec<i32>) -> i32 {
    // Start with mutable reference, return immutable analysis
    mixed_chain_modify(data);
    mixed_chain_analyze(data)
}

fn mixed_chain_modify(data: &mut Vec<i32>) {
    // TODO: Modify the data
}

fn mixed_chain_analyze(data: &Vec<i32>) -> i32 {
    // TODO: Analyze the data (immutable)
    0
}
