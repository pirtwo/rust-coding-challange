fn main() {
    let primes = eratosthenes_sieve(2000000);
    let sum: u64 = primes.iter().map(|x| x.value).sum();
    println!("sum of primes below 2 million is = {}", sum);
}

fn eratosthenes_sieve(up_to: u64) -> Vec<Number> {
    let mut numbers = vec![];
    let upper_bound = ((up_to - 2) as f64).sqrt().floor() as usize;

    for i in 2..=up_to {
        numbers.push(Number::new(i, true));
    }

    for i in 0..=upper_bound {
        if numbers[i].is_prime {
            let prime = numbers[i].value;
            let mut idx = prime.pow(2) - 2;
            loop {
                if idx < numbers.len() as u64 {
                    numbers[idx as usize].is_prime = false;
                    idx += prime;
                } else {
                    break;
                }
            }
        }
    }

    numbers.retain(|x| x.is_prime);
    numbers
}

#[derive(Debug)]
struct Number {
    value: u64,
    is_prime: bool,
}

impl Number {
    fn new(value: u64, is_prime: bool) -> Self {
        Number { value, is_prime }
    }
}
