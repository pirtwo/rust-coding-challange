fn main() {
    let n = 1;
    let m = 20;
    let res = smallest_divisible(n, m);
    print!(
        "smallest divisible number between {} and {} = {}",
        n, m, res
    );
}

// find a smallest number that is evenly divisible to
// all integer numbers between n to m.
fn smallest_divisible(n: u32, m: u32) -> u32 {
    let mut flag = false;

    for i in m..=999999999 {
        for j in n..=m {
            if i % j > 0 {
                flag = false;
                break;
            }
            flag = true;
        }
        if flag {
            return i;
        }
    }

    return 1;
}
