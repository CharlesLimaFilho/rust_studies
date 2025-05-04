pub fn run() {
    println!("Yo");

    println!("Number: {}", 1);

    // Basic Formatting
    println!("{} is from {}", "Lucas", "Manaus");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes {2}", "Joao", "Bahia", "fanta uva");

    // Named Arguments
    println!("{name} likes to play {game}", name = "Marcelo", game = "God of War");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}