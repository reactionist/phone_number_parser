extern crate phone_number_parser;

use phone_number_parser::parse_phone_number;
use phone_number_parser::ParsePhoneNumberError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone_number() {
        let phone_number = "+380977621906";
        let result = parse_phone_number(phone_number);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_phone_number_with_operator() {
        let phone_number = "+380979663406";
        let result = parse_phone_number(phone_number);
        assert!(result.is_ok(), "The phone number should be valid.");
        if let Ok(parsed) = result {
            assert_eq!(parsed.operator, "97", "Operator code should be 50");
            assert_eq!(
                parsed.operator_name, "Kyivstar",
                "Operator name should be Vodafone"
            );
            assert_eq!(
                parsed.subscriber_number, "9663406",
                "Subscriber number should be 970663406"
            );
        }
    }

    #[test]
    fn test_valid_phone_number_region() {
        let phone_number = "+380441234567";
        let result = parse_phone_number(phone_number);
        assert!(
            result.is_ok(),
            "The phone number should be valid with an area code."
        );
        if let Ok(parsed) = result {
            assert_eq!(parsed.operator, "44", "Area code should be 44");
            assert_eq!(
                parsed.operator_name, "Kyiv City",
                "Area name should be Kyiv City"
            );
        }
    }

    #[test]
    fn test_empty_phone_number() {
        let phone_number = "";
        let result = parse_phone_number(phone_number);
        assert!(matches!(
            result,
            Err(ParsePhoneNumberError::InvalidInput(_))
        ));
    }

    #[test]
    fn test_phone_number_with_invalid_characters() {
        let phone_number = "+50abcde12345";
        let result = parse_phone_number(phone_number);
        assert!(matches!(
            result,
            Err(ParsePhoneNumberError::InvalidInput(_))
        ));
    }
    #[test]
    fn test_phone_number_with_invalid_format() {
        let phone_number = "12345";
        let result = parse_phone_number(phone_number);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_long_phone_number() {
        let phone_number = "+501234567890123456789";
        let result = parse_phone_number(phone_number);
        assert!(
            matches!(result, Err(ParsePhoneNumberError::InvalidInput(_))),
            "The phone number should be invalid because it is too long."
        );
    }

    #[test]
    fn test_phone_number_with_special_characters() {
        let phone_number = "+50-123-4567";
        let result = parse_phone_number(phone_number);
        assert!(
            matches!(result, Err(ParsePhoneNumberError::InvalidInput(_))),
            "The phone number should be invalid due to special characters."
        );
    }

    #[test]
    fn test_phone_number_with_letters() {
        let phone_number = "+50ABC1234567";
        let result = parse_phone_number(phone_number);
        assert!(
            matches!(result, Err(ParsePhoneNumberError::InvalidInput(_))),
            "The phone number should be invalid due to letters."
        );
    }

    #[test]
    fn test_phone_number_with_whitespace() {
        let phone_number = "+50 123 4567";
        let result = parse_phone_number(phone_number);
        assert!(
            matches!(result, Err(ParsePhoneNumberError::InvalidInput(_))),
            "The phone number should be invalid due to whitespace."
        );
    }
}
