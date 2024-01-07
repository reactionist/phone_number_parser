use pest::Parser;
use pest_derive::Parser;
use serde::Serialize;
use std::fmt;

#[derive(Parser)]
#[grammar = "phone_number.pest"]
pub struct PhoneNumberParser;

#[derive(Debug, Serialize)]
pub struct ParsedPhoneNumber {
    pub operator: String,
    pub operator_name: String,
    pub subscriber_number: String,
}

#[derive(Debug)]
pub enum ParsePhoneNumberError {
    InvalidInput(String),
}

impl fmt::Display for ParsePhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsePhoneNumberError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for ParsePhoneNumberError {}

pub fn parse_phone_number(
    phone_number_string: &str,
) -> Result<ParsedPhoneNumber, ParsePhoneNumberError> {
    if phone_number_string.is_empty() {
        return Err(ParsePhoneNumberError::InvalidInput(
            "Phone number is empty".to_string(),
        ));
    }

    if phone_number_string
        .chars()
        .any(|c| !c.is_ascii_digit() && c != '+')
    {
        return Err(ParsePhoneNumberError::InvalidInput(
            "Phone number contains invalid characters".to_string(),
        ));
    }

    let pairs =
        PhoneNumberParser::parse(Rule::phone_number, phone_number_string).map_err(|_| {
            ParsePhoneNumberError::InvalidInput("Error parsing phone number".to_string())
        })?;

    let mut parsed_phone_number = ParsedPhoneNumber {
        operator: String::new(),
        operator_name: String::new(),
        subscriber_number: String::new(),
    };

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::operator_code => {
                    parsed_phone_number.operator = inner_pair.as_str().to_string();
                    match get_operator_or_area_name(&parsed_phone_number.operator) {
                        Ok(name) => parsed_phone_number.operator_name = name,
                        Err(_) => {
                            return Err(ParsePhoneNumberError::InvalidInput(
                                "Invalid operator or area code".to_string(),
                            ))
                        }
                    }
                }
                Rule::subscriber_number => {
                    parsed_phone_number.subscriber_number = inner_pair.as_str().to_string()
                }
                _ => {}
            }
        }
    }

    Ok(parsed_phone_number)
}

fn get_operator_or_area_name(code: &str) -> Result<String, ParsePhoneNumberError> {
    match code {
        "50" | "66" | "95" | "99" => Ok("Vodafone".to_string()),
        "63" | "73" | "93" => Ok("lifecell".to_string()),
        "67" | "68" | "96" | "97" | "98" => Ok("Kyivstar".to_string()),
        "91" => Ok("3mob".to_string()),
        "92" => Ok("PEOPLEnet".to_string()),
        "94" => Ok("Intertelecom".to_string()),

        "44" => Ok("Kyiv City".to_string()),
        "32" => Ok("Lviv Oblast".to_string()),
        "48" => Ok("Odesa Oblast".to_string()),
        "56" => Ok("Dnipro".to_string()),
        "61" => Ok("Zaporizhzhia Oblast".to_string()),
        "64" => Ok("Luhansk Oblast".to_string()),
        "69" => Ok("Sevastopol".to_string()),
        "45" => Ok("Kyiv Oblast".to_string()),
        "46" => Ok("Chernihiv Oblast".to_string()),
        "47" => Ok("Cherkasy Oblast".to_string()),
        "51" => Ok("Mykolaiv Oblast".to_string()),
        "52" => Ok("Kirovohrad Oblast".to_string()),
        "53" => Ok("Poltava Oblast".to_string()),
        "54" => Ok("Sumy Oblast".to_string()),
        "55" => Ok("Kherson Oblast".to_string()),

        _ => Err(ParsePhoneNumberError::InvalidInput(format!(
            "Invalid operator or area code: {}",
            code
        ))),
    }
}
