pub fn is_prime(n: u64) -> bool {
    n > 1 && {
        let limit = (n as f64).sqrt() as u64 + 1;
        let mut range = 2..limit;
        !range.any(|i| n % i == 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
    }
}
