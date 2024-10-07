pub fn add(left: i64, right: i64) -> i64 {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert_eq!(3, super::add(1, 2));
    }
}
