use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get array len
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}