pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .filter(|&&apple| {
            let position = a + apple;
            position >= s && position <= t
        })
        .count() as i32;

    let orange_count = oranges
        .iter()
        .filter(|&&orange| {
            let position = b + orange;
            position >= s && position <= t
        })
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let result =
            count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);

        assert_eq!(result, (1, 1));
    }
}