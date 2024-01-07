# Phone Number Parser

## Overview

The Phone Number Parser is a Rust application designed to parse and provide detailed information about Ukrainian phone numbers. It uses the `pest` parser and `serde` for serialization, offering comprehensive functionality for parsing phone numbers into their respective components and validating them.

## Features

- Parse Ukrainian phone numbers and extract details like operator, operator name, and subscriber number.
- Validate the input phone number format.
- Display information about the type of number (Mobile Operator or Area Code).
- Append parsed phone number details to a CSV file for record-keeping.

## Parsing Rules

The parsing rules are defined using the `pest` grammar. 

```bash
phone_number = { "+" ~ "380" ~ (operator_code) ~ subscriber_number }
operator_code = { "67" | "68" | "96" | "97" | "98" | "50" | "66" | "95" | "99" | "63" | "73" | "93" | "44" | "32" | "48" | "56" | "61" | "64" | "69" | "45" | "46" | "47" | "51" | "52" | "53" | "54" | "55" }
subscriber_number = { ASCII_DIGIT{7} }
ASCII_DIGIT = { '0'..'9' }
```

## Error Handling

The application handles various error scenarios, including invalid input, empty phone number, invalid characters, and unrecognized operator or area codes.

## Prerequisites

To run this application, ensure you have Rust and Cargo installed on your machine.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/reactionist/phone_number_parser.git
cd phone_number_parser
make build
```

## Usage

The application can be executed from the command line. Use the following command to parse a phone number:

```bash
make parse PHONE_NUMBER=[phone_number]
```

Replace `[phone_number]` with the actual phone number you want to parse.

Example:

```bash
make parse PHONE_NUMBER=+380977621905
```

To show information about how to use Phone Number parser CLI run `help`.
