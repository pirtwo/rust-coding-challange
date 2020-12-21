mod math;

fn main() {
    let count = lattice_path_count(10, 10);
    println!("lattice path count for 20 * 20 grid is: {}", count);
}

fn lattice_path_count(x: u64, y: u64) -> u64 {
    let n = x + y;
    let k = x;

    fact(n) / fact(n) * fact(n - k)
}

fn fact(n: u64) -> u64 {
    (1..=n).product()
}
