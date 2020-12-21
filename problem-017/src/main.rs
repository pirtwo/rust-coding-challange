use std::collections::HashMap;

fn main() {
    let mut words = String::new();
    for i in 1..=1000 {
        words.push_str(number_to_words(i as i32).as_str());
    }
    let count = words.chars().filter(|&x| x != ' ' && x != '-').count();
    println!(
        "all numbers from 1 to 1000 in words have {} letters.",
        count
    );
}

fn number_to_words(n: i32) -> String {
    let mut num = n as usize;
    let mut round = 0u32;
    let mut words = vec![];
    let dic: HashMap<usize, &str> = [
        (0, ""),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (1000, "thousand"),
        (1000000, "million"),
        (1000000000, "billion"),
    ]
    .iter()
    .cloned()
    .collect();

    while num != 0 {
        let chunk = num % 1000;
        let ones = chunk % 10;
        let tens = (chunk % 100) / 10;
        let hundreds = chunk / 100;

        if round > 0 && chunk > 0 {
            words.push(dic.get(&(1000usize.pow(round))).unwrap().to_string());
        }

        if ones > 0 && tens == 0 {
            words.push(dic.get(&(ones)).unwrap().to_string());
        } else if tens == 1 {
            words.push(dic.get(&(ones + 10)).unwrap().to_string());
        } else if tens > 1 {
            words.push({
                if ones == 0 {
                    dic.get(&(tens * 10)).unwrap().to_string()
                } else {
                    format!(
                        "{}-{}",
                        dic.get(&(tens * 10)).unwrap().to_string(),
                        dic.get(&(ones)).unwrap().to_string()
                    )
                }
            });
        }

        if hundreds > 0 {
            words.push(format!(
                "{} hundred{}",
                dic.get(&(hundreds)).unwrap().to_string(),
                {
                    if tens > 0 || ones > 0 {
                        " and".to_string()
                    } else {
                        "".to_string()
                    }
                }
            ));
        }

        num = num / 1000;
        round += 1;
    }
    words.reverse();
    words.join(" ")
}
