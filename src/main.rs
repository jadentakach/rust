fn get_input(prompt: &str) -> String {
    let mut input: String = String::new();
    println!("{}", prompt);

    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    input
}

fn ac_factor(b: i32, c_factors: Vec<Vec<i32>>) -> Vec<i32> {
    for factor_pair in c_factors {
        if factor_pair[0] + factor_pair[1] == b {
            return factor_pair;
        }
    }

    return vec![0, 0];
}

fn factors(int: i32) -> Vec<Vec<i32>> {
    let mut listed_factors: Vec<Vec<i32>> = Vec::new();
    let n = int.abs();

    for i in 1..=n {
        if int % i == 0 {
            let j: i32 = int / i;

            if !listed_factors.contains(&vec![i, j]) {
                listed_factors.push(vec![i, j]);
            }

            if !listed_factors.contains(&vec![-i, -j]) {
                listed_factors.push(vec![-i, -j]);
            }
        }
    }

    listed_factors
}

struct Polynomial {
    a: i32,
    b: i32,
    c: i32
}

impl Polynomial {
    fn factor(self) -> Vec<i32> {
        let c_factors: Vec<Vec<i32>> = factors(self.c);
        let end_factor: Vec<i32> = ac_factor(self.b, c_factors);

        end_factor
    }
}

fn generate_polynomial(term_stack: &String) -> Polynomial {
    let mut terms: Vec<i32> = Vec::new();
    
    for term in term_stack.trim().split_whitespace() {
        if !term.contains("+") && !term.contains("- ") {
            let pushable_term: i32 = term.parse::<i32>().unwrap();
            terms.push(pushable_term);
        }
    }

    Polynomial { a: terms[0], b: terms[1], c: terms[2] }
}

fn format_factor(factor: Vec<i32>) -> String {
    let a = factor[0];
    let b = factor[1];

    if a > 0 && factor[1] > 0 {
        return format!("(x+{})(x+{})", a, b);
    } 

    if a < 0 && factor[1] < 0 {
        return format!("(x{})(x{})", a, b);
    }

    let positive: i32 = if a > 0 { a } else { b };
    let negative: i32 = if a < 0 { a } else { b };

    return format!("(x+{})(x{})", positive, negative);
}

fn main() {
    let input: String = get_input("Input polynomial: ");
    let polynomial: Polynomial = generate_polynomial(&input);
    let factored: Vec<i32> = polynomial.factor();
    println!("{}", format_factor(factored));

    /*

    println!("Factors of {}:", int);
    let factors: Vec<Vec<i32>> = factors(int);
    for factor in &factors {
        println!("[{}, {}]", factor[0], factor[1]);
    }

    let gcf = ac_factor(b, factors);
    

    */
}