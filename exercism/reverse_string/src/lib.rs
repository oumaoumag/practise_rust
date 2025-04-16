pub fn reverse_str(Str: &str) -> String {
    Str.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!("", reverse_str(""));
    }

    #[test]
    fn test_single_word() {
        assert_eq!("stressed", reverse_str("desserts"));
        assert_eq!("sports", reverse_str("strops"));
        assert_eq!("niwdoG", reverse_str("Godwin"));
    }

    
    #[test]
    fn test_palindrome() {
        assert_eq!("racecar", reverse_str("racecar"));
        assert_eq!("noon", reverse_str("noon"));
    }

    #[test]
    fn test_with_space() {
        assert_eq!("oof rab", reverse_str("bar foo"));
    }

    #[test]
    fn test_unicode() {
        assert_eq!("🦀💻", reverse_str("💻🦀"));
        assert_eq!("你好", reverse_str("好你"));
    }
}