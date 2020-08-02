fn main() {
    let n = 40;
    let res = even_fibo_sum(n);
    print!("The sum of even values of fibonacci({}) = {}", n, res);
}

// finds the sum of even fibo numbers between 0 and n.
fn even_fibo_sum(n: u32) -> u32 {
    // fibo(n) = fibo(n-1) + fibo(n-2)
    // 1, 1, 2, 3, 5, 8, 13, ...

    let mut curr; // fibo(i)
    let mut n1 = 1; // n-1
    let mut n2 = 0; // n-2
    let mut sum = 0;

    for _i in 1..=n {
        curr = n1 + n2;
        n2 = n1;
        n1 = curr;

        if curr % 2 == 0 {
            sum += curr;
        }
    }

    return sum;
}
