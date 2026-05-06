use std::io::{self, BufRead};

pub fn bon_appetit(bill: &[i32], k: i32, b: i32) -> String {
    let total_without_anna: i32 = bill
        .iter()
        .enumerate()
        .filter(|&(index, _)| index != k as usize)
        .map(|(_, &price)| price)
        .sum();

    let anna_share = total_without_anna / 2;

    if b == anna_share {
        "Bon Appetit".to_string()
    } else {
        (b - anna_share).to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    println!("{}", bon_appetit(&bill, k, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_refund() {
        let bill = vec![3, 10, 2, 9];

        assert_eq!(bon_appetit(&bill, 1, 12), "5");
    }

    #[test]
    fn test_bon_appetit_fair() {
        let bill = vec![3, 10, 2, 9];

        assert_eq!(bon_appetit(&bill, 1, 7), "Bon Appetit");
    }
}
