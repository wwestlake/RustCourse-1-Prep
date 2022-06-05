#[cfg(test)]
mod test {
    #[test]
    pub fn test_1() {
        assert_eq!(1, 1);
    }

    #[test]
    pub fn failing_test() {
        assert_eq!(1, 2);
    }

}


