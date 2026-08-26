fn main() {
    println!("Input problem:");
    let mut problem: String = String::new();
    std::io::stdin().read_line(&mut problem).expect("Failed to read input");
    println!("Problem: {}", problem);
}