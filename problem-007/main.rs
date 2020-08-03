fn main() {
    let res = sieve_of_eratosthenes(2000000);
    print!("found {} prime number. \n", res.len());
    print!("10001th prime number is: {} \n", res[10001]);
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut seq: Vec<Number> = Vec::new();
    let mut res: Vec<usize> = Vec::new();

    for i in 2..=n {
        seq.push(Number {
            value: i,
            is_prime: true,
        });
    }

    let end = (seq.len() as f32).sqrt().floor() as usize;

    for i in 0..=end {
        if seq[i].is_prime {
            let mut index: usize = seq[i].value.pow(2) - 2;
            loop {
                if index >= seq.len() {
                    break;
                } else {
                    seq[index].is_prime = false;
                    index += seq[i].value;
                }
            }
        }
    }

    for i in 0..seq.len() {
        if seq[i].is_prime {
            res.push(seq[i].value);
        }
    }

    return res;
}

struct Number {
    value: usize,
    is_prime: bool,
}
