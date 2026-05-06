use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut best_id = 1;
    let mut best_count = counts[1];

    for id in 2..=5 {
        if counts[id] > best_count {
            best_count = counts[id];
            best_id = id;
        }
    }

    best_id as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_example_1() {
        let arr = vec![1, 1, 2, 2, 3];

        assert_eq!(migratory_birds(&arr), 1);
    }

    #[test]
    fn test_migratory_birds_example_2() {
        let arr = vec![1, 4, 4, 4, 5, 3];

        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_smallest_id_on_tie() {
        let arr = vec![2, 2, 3, 3, 4];

        assert_eq!(migratory_birds(&arr), 2);
    }
}
