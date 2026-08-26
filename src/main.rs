fn main() {
    println!("Input problem:");
    let mut problem: String = String::new();
    std::io::stdin().read_line(&mut problem).expect("Failed to read input");
    println!("Problem: {}", problem);

    let mut elements: Vec<String> = Vec::new();

    for part in problem.split(" + ") {
        elements.push(String::from(part));
    }

    println!("A: {}", elements[0]);
    println!("B: {}", elements[1]);
    println!("C: {}", elements[2]);
}