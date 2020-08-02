fn main() {
    let n = 1000;
    let res = find_sum(n);
    print!(
        "The sum of all the multiples of 3 or 5 below {} = {}",
        n, res
    );
}

// find sum of all the multiples of 3 or 5 from 0 to n
fn find_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..=n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    return sum;
}
