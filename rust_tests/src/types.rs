pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let r: i64 = 98498149819;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);
    println!("Max u32: {}", std::u32::MAX);
    println!("Max u64: {}", std::u64::MAX);
    println!("Max u128: {}", std::u128::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, r, is_active, is_greater, a1, face))
}