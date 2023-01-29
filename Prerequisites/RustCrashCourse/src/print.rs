pub fn run() {
    println!("hello from print.rs file");
    println!("Number: {}", 1);
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
    println!("{name} likes to play {activity}", name = "Brad", activity = "code");
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "str", 0.212));
    println!("10 + 10 = {:?}", 10 + 10);
}