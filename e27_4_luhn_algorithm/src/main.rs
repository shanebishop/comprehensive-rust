pub fn luhn(cc_number: &str) -> bool {
    let mut num_digits = 0;
    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        num_digits += 1;
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else if c.is_whitespace() {
            continue;
        } else {
            return false;
        }
    }

    num_digits >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
        assert!(!luhn("1234 5678 9012 3456"));
        assert!(!luhn("1111 1111 1111 1111"));
    }

    #[test]
    fn test_valid_cc_number_with_spaces() {
        assert!(luhn("4 2 6 3 9 8 2 6 4 0 2 6 9 2 9 9"));
        assert!(luhn("4 5 3 9 3 1 9 5 0 3 4 3 6 4 6 7"));
    }

    #[test]
    fn test_empty_string() {
        assert!(!luhn(""));
    }

    #[test]
    fn test_non_digit_characters() {
        assert!(!luhn("foo"));
        assert!(!luhn("foo 0 0"));
        assert!(!luhn("4263 9826 4026 929x"));
        assert!(!luhn("4539 3195 0343 64a7"));
    }

    #[test]
    fn test_single_digit() {
        assert!(!luhn("4"));
        assert!(!luhn("0"));
    }
}
