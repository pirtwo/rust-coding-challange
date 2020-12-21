pub fn product(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut a = a.clone();
    let mut b = b.clone();
    let mut product = vec![0u8];

    a.reverse();
    b.reverse();

    let a_padding = {
        if a.len() < b.len() {
            b.len() - a.len()
        } else {
            0
        }
    };

    let b_padding = {
        if b.len() < a.len() {
            a.len() - b.len()
        } else {
            0
        }
    };

    // add zero padding based on difference between numA and numB length
    a.append(&mut vec![0u8; a_padding]);
    b.append(&mut vec![0u8; b_padding]);

    a.iter().enumerate().for_each(|(i, x)| {
        let mut acc = vec![];
        let mut carry = 0;

        b.iter().for_each(|y| {
            let res = x * y + carry;
            carry = res / 10;
            acc.push(res % 10);
        });

        if carry > 0 {
            acc.push(carry);
        }

        acc.reverse();
        acc.append(&mut vec![0u8; i]);
        product = sum(&product, &acc);
        acc.clear();
    });

    product
}

pub fn sum(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut a = a.clone();
    let mut b = b.clone();
    let mut carry = 0;
    let mut sum = vec![];

    a.reverse();
    b.reverse();

    let a_padding = {
        if a.len() < b.len() {
            b.len() - a.len() + 1
        } else {
            1
        }
    };

    let b_padding = {
        if b.len() < a.len() {
            a.len() - b.len() + 1
        } else {
            1
        }
    };

    // add zero padding based on difference between numA and
    // numB length, also add one extra zero for possible carry.
    a.append(&mut vec![0u8; a_padding]);
    b.append(&mut vec![0u8; b_padding]);

    a.iter().enumerate().for_each(|(i, x)| {
        let acc = x + b[i] + carry;
        carry = acc / 10;
        sum.push(acc % 10);
    });

    sum.reverse();

    // clear zero padding
    sum.into_iter().skip_while(|x| x == &0_u8).collect()
}
