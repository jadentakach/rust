fn get_input(prompt: &str) -> String {
    let mut input: String = String::new();
    println!("{}", prompt);

    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    input
}

fn factors(int: i32) -> Vec<Vec<i32>> {
    let mut listed_factors: Vec<Vec<i32>> = Vec::new();
    for i in 1..=int {
        for j in 1..=int {
            if i * j == int && !listed_factors.contains(&(vec![i, j])) && !listed_factors.contains(&(vec![j, i])) {
                listed_factors.push(vec![i, j]);
            }
        }
    }

    return listed_factors;
}

fn main() {
    let mut input: String = get_input("Input number: ");
    let int: i32 = input.trim().parse::<i32>().unwrap();
    input.clear();

    println!("Factors of {}:", int);
    let factors: Vec<Vec<i32>> = factors(int);
    for factor in factors {
        println!("[{}, {}]", factor[0], factor[1]);
    }
}