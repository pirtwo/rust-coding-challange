fn main() {
    let res = palindrome();
    print!(
        "The bigest palindrome number is {} * {} = {}",
        res.0, res.1, res.2
    );
}

// return largest palindrome number
// of two 3-digit number multiplication
fn palindrome() -> (u32, u32, u32) {
    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let num = (i * j).to_string();
            let rev = num.chars().rev().collect::<String>();
            if num == rev {
                return (i, j, i * j);
            }
        }
    }

    return (0, 0, 0);
}
