use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple = ((grade / 5) + 1) * 5;

                if next_multiple - grade < 3 {
                    next_multiple
                } else {
                    grade
                }
            }
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students_example() {
        let grades = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];

        assert_eq!(grading_students(&grades), expected);
    }

    #[test]
    fn test_grading_students_below_38() {
        let grades = vec![10, 20, 37];
        let expected = vec![10, 20, 37];

        assert_eq!(grading_students(&grades), expected);
    }

    #[test]
    fn test_grading_students_rounding() {
        let grades = vec![84, 29, 57];
        let expected = vec![85, 29, 57];

        assert_eq!(grading_students(&grades), expected);
    }
}
