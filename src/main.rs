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
    fn factor() -> Vec<i32> {
        vec![0, 1] // temp
    }
}

fn generate_polynomial(term_stack: &String) -> Polynomial {
    let mut terms: Vec<i32> = Vec::new();
    
    for term in term_stack.split_whitespace() {
        if !term.contains("+") && !term.contains("- ") {
            let pushable_term: i32 = term.trim().parse::<i32>().unwrap();
            terms.push(pushable_term);
        }
    }

    Polynomial { a: terms[0], b: terms[1], c: terms[2] }
}

fn main() {
    let mut input: String = get_input("Input polynomial: ");
    let polynomial: Polynomial = generate_polynomial(&input);
    input.clear();

    /*

    println!("Factors of {}:", int);
    let factors: Vec<Vec<i32>> = factors(int);
    for factor in &factors {
        println!("[{}, {}]", factor[0], factor[1]);
    }

    let gcf = ac_factor(b, factors);
    if gcf[0] > 0 && gcf[1] > 0 {
        println!("(x+{})(x+{})", gcf[0], gcf[1]);
        return;
    } 

    if gcf[0] < 0 && gcf[1] < 0 {
        println!("(x{})(x{})", gcf[0], gcf[1]);
        return;
    }

    let positive: i32 = if gcf[0] > 0 { gcf[0] } else { gcf[1] };
    let negative: i32 = if gcf[0] < 0 { gcf[0] } else { gcf[1] };
    println!("(x+{})(x{})", positive, negative);

    */
}