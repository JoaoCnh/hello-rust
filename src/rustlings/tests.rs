pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn times_two(num: i32) -> i32 {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(true);
    }

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(1,1);
    }

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn returns_twice_of_positive_numbers() {
        assert_eq!(times_two(2), 4);
    }

    #[test]
    fn returns_twice_of_negative_numbers() {
        assert_eq!(times_two(-2), -4);
    }
}