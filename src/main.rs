use clap::{App, Arg, SubCommand};
use phone_number_parser::parse_phone_number;
use std::fs::OpenOptions;
use std::process;
use csv::WriterBuilder;

fn main() {
    let matches = App::new("Phone Number Parser")
        .version("1.0")
        .author("reactionist")
        .about("Parses and provides information about Ukrainian phone numbers")
        .subcommand(SubCommand::with_name("parse")
            .about("Parses a phone number")
            .arg(Arg::with_name("phone_number")
                .help("The phone number to parse")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            let phone_number = sub_m.value_of("phone_number").unwrap();
            match parse_phone_number(phone_number) {
                Ok(parsed_number) => {
                    println!("The number {} has been parsed and is valid.", phone_number);
                    match parsed_number.operator_name.as_ref() {
                        "Vodafone" | "lifecell" | "Kyivstar" | "3mob" | "PEOPLEnet" | "Intertelecom" => {
                            println!("Mobile Operator: {}", parsed_number.operator_name);
                        },
                        _ => {
                            println!("Area Code: {}", parsed_number.operator_name);
                        }
                    }

                    // Append to CSV file
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("parsed_phone_numbers.csv")
                        .unwrap();

                    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
                    wtr.serialize(&parsed_number).unwrap();
                    wtr.flush().unwrap();

                    println!("Result appended to parsed_phone_numbers.csv");
                },
                Err(err) => {
                    eprintln!("Failed to parse phone number: {}", err);
                    process::exit(1);
                }
            }
        },
        _ => println!("Invalid command. Use 'help' for more information."),
    }
}
