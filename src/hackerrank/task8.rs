use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_count += 1;
        }

        if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let scores: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_example_1() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        let expected = vec![2, 4];

        assert_eq!(breaking_records(&scores), expected);
    }

    #[test]
    fn test_breaking_records_example_2() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        let expected = vec![4, 0];

        assert_eq!(breaking_records(&scores), expected);
    }
}
