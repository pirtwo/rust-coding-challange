fn main() {
    let mut seq_num = 0;
    let mut curr_len;
    let mut max_len = 0;

    for i in 1..1000000 {
        curr_len = collatz_seq_len(i);
        if curr_len > max_len {
            seq_num = i;
            max_len = curr_len;
        }
    }

    let seq = collatz_seq(seq_num);

    println!(
        "max collatz sequence length is: {} for number: {}\nsequence is: {:#?}",
        max_len, seq_num, seq
    );
}

fn collatz_seq(n: u64) -> Vec<u64> {
    let mut seq = vec![];
    let mut curr = n;
    seq.push(curr);

    loop {
        curr = match curr % 2 == 0 {
            true => curr / 2,
            false => 3 * curr + 1,
        };

        seq.push(curr);

        if curr == 1 {
            break;
        }
    }

    seq
}

// n -> n/2  (n is even)
// n -> 3n+1 (n is odd)
fn collatz_seq_len(n: u64) -> u64 {
    let mut len = 1;
    let mut curr = n;

    loop {
        curr = match curr % 2 == 0 {
            true => curr / 2,
            false => 3 * curr + 1,
        };

        if curr == 1 {
            break;
        }

        len += 1;
    }

    len
}
