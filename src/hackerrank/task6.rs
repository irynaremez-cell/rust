pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            return "YES".to_string();
        }

        return "NO".to_string();
    }

    let distance = x2 - x1;
    let speed_difference = v1 - v2;

    if distance % speed_difference == 0 && distance / speed_difference >= 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }
}