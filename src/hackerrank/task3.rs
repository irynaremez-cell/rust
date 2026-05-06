pub fn sum_digits(x: i32) -> i32 {
    let mut n = x;
    let mut sum = 0;

    while n > 0 {
        sum += n % 10;
        n /= 10;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(123), 6);
        assert_eq!(sum_digits(0), 0);
        assert_eq!(sum_digits(999), 27);
    }
}