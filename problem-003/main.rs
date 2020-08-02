fn main() {
    let num: u64 = 600851475142;
    let res: Vec<u64> = prime_factors(num);
    print!("prime factors of {} = {:?}", num, res);
}

// find smallest divisible number to n, the n is prime if fn returns 1.
fn find_divisible(n: u64) -> u64 {
    for i in 2..n {
        if n % i == 0 {
            return i;
        }
    }

    return 1;
}

// returns the prime factors of n.
fn prime_factors(n: u64) -> Vec<u64> {
    let mut curr: u64;
    let mut f1: u64;
    let mut f2: u64;
    let mut factors_tree: Vec<u64> = Vec::new();
    let mut prime_factors: Vec<u64> = Vec::new();

    factors_tree.push(n);

    loop {
        curr = factors_tree.pop().expect("error");
        f1 = find_divisible(curr);
        f2 = curr / f1;

        if f1 == 1 {
            prime_factors.push(curr);
        } else {
            factors_tree.push(f1);
            factors_tree.push(f2);
        }

        if factors_tree.is_empty() {
            break;
        }
    }

    return prime_factors;
}
