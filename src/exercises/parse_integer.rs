//! G005: Parse a signed decimal integer with an explicit error result.

use std::num::ParseIntError;

/// Return the parsed i32 on success, or a ParseIntError on invalid input.
/// Accept an optional leading ASCII '+' or '-' followed by ASCII digits.
/// Leading zeros are allowed; whitespace, separators, and other syntax are not.
/// Values outside the i32 range must return Err, not panic or wrap.
/// Standard-library parsing is allowed and encouraged for this exercise.
pub fn parse_integer(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

#[cfg(test)]
mod tests {
    use super::parse_integer;

    #[test]
    fn parses_positive_integer() {
        assert_eq!(parse_integer("42"), Ok(42));
    }

    #[test]
    fn parses_negative_integer() {
        assert_eq!(parse_integer("-42"), Ok(-42));
    }

    #[test]
    fn parses_explicit_plus_sign() {
        assert_eq!(parse_integer("+42"), Ok(42));
    }

    #[test]
    fn parses_zero_with_either_sign() {
        for input in ["0", "+0", "-0"] {
            assert_eq!(parse_integer(input), Ok(0), "input: {input:?}");
        }
    }

    #[test]
    fn accepts_leading_zeros() {
        assert_eq!(parse_integer("00042"), Ok(42));
        assert_eq!(parse_integer("-00042"), Ok(-42));
        assert_eq!(parse_integer("+000"), Ok(0));
    }

    #[test]
    fn accepts_both_integer_limits() {
        assert_eq!(parse_integer("2147483647"), Ok(i32::MAX));
        assert_eq!(parse_integer("-2147483648"), Ok(i32::MIN));
    }

    #[test]
    fn rejects_values_just_outside_integer_limits() {
        assert!(parse_integer("2147483648").is_err());
        assert!(parse_integer("-2147483649").is_err());
    }

    #[test]
    fn rejects_very_large_magnitudes_without_panicking() {
        assert!(parse_integer("999999999999999999999999999999").is_err());
        assert!(parse_integer("-999999999999999999999999999999").is_err());
    }

    #[test]
    fn rejects_empty_input_and_signs_without_digits() {
        for input in ["", "+", "-"] {
            assert!(parse_integer(input).is_err(), "input: {input:?}");
        }
    }

    #[test]
    fn rejects_whitespace_in_any_position() {
        for input in [" ", " 42", "42 ", "4 2", "\t42", "42\n"] {
            assert!(parse_integer(input).is_err(), "input: {input:?}");
        }
    }

    #[test]
    fn rejects_repeated_or_embedded_signs() {
        for input in ["++42", "--42", "+-42", "-+42", "4-2", "4+2"] {
            assert!(parse_integer(input).is_err(), "input: {input:?}");
        }
    }

    #[test]
    fn rejects_non_decimal_syntax_and_trailing_junk() {
        for input in ["abc", "42x", "3.5", "1e3", "0x2a", "1_000", "1,000"] {
            assert!(parse_integer(input).is_err(), "input: {input:?}");
        }
    }

    #[test]
    fn rejects_non_ascii_digits_and_signs() {
        for input in ["\u{ff14}\u{ff12}", "\u{0664}\u{0662}", "\u{2212}42"] {
            assert!(parse_integer(input).is_err(), "input: {input:?}");
        }
    }
}
