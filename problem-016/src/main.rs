mod math;

fn main() {
    let mut pow = vec![2u8];

    for _ in 1..1000 {
        pow = math::vec::product(&pow, &vec![2u8]);
    }

    let sum: u32 = pow.iter().map(|x| *x as u32).sum();

    println!(
        "sum of digits of 2^1000 is: {}\nresult: {:?}",
        sum,
        pow.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
