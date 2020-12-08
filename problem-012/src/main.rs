fn main() {
    let mut tri_num = 1;
    let mut fct_num;

    for i in 1..50000 {
        tri_num = triangular_number(i);
        fct_num = factors_count(tri_num);
        if fct_num >= 500 {
            break;
        }
    }

    let mut factors = factors(tri_num);
    factors.sort();

    println!(
        "Highly divisible triangular number: {} with {} factors.",
        tri_num,
        factors.len()
    );
    println!("Factors: {:#?}", factors);
}

fn triangular_number(nth: u64) -> u64 {
    (nth * (nth + 1)) / 2
}

fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    for i in 1..=(n as f64).sqrt().floor() as u64 {
        if n % i == 0 {
            factors.push(i);
            factors.push(n / i);
        }
    }
    factors
}

fn factors_count(n: u64) -> u64 {
    let mut count = 0;
    for i in 1..=(n as f64).sqrt().floor() as u64 {
        if n % i == 0 {
            count += 1;
        }
    }
    count
}
