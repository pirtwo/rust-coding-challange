fn main() {
    // rule: a > b > c
    // rule: a^2 + b^2 = c^2
    // rule: a + b + c = 1000
    
    let (a, b, c) = find_spt();
    print!("a: {}, b: {}, c: {}", a, b, c);
}

// find special pythagorean triplet
fn find_spt() -> (usize, usize, usize) {
    let n: usize = 1000;
    
    for a in 1..=n {
        for b in (a + 1)..=n {
            // c = sqrt(a^2 + b^2)
            let c = ((a.pow(2) + b.pow(2)) as f64).sqrt() as usize;
            if (a + b + c == 1000) && (a.pow(2) + b.pow(2) == c.pow(2)) && c > b {
                return (a, b, c);
            }
        }
    }
    (0, 0, 0)
}
