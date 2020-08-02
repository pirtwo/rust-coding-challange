fn main() {
    let n = 1000;
    let res1 = square_of_sums(n);
    let res2 = sum_of_squares(n);
    print!(
        "square_of_sums({0}) - sum_of_squares({0}) = {1} - {2} = {3}",
        n,
        res1,
        res2,
        res1 - res2
    );
}

// returns square sum of values between 1 and n.
fn square_of_sums(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += i;
    }

    return sum.pow(2);
}

// returns sum of square values between 1 and n.
fn sum_of_squares(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += i.pow(2);
    }

    return sum;
}
