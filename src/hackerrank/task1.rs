<<<<<<< HEAD
fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
=======
pub fn solve(a: i32, b: i32) -> i32 {
    a + b
>>>>>>> 0a80e1c2dffcaa97ea5ac54bcc2c0d5af7bee4b4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
    fn test_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
=======
    fn test_solve() {
        assert_eq!(solve(2, 3), 5);
>>>>>>> 0a80e1c2dffcaa97ea5ac54bcc2c0d5af7bee4b4
    }
}