#![allow(unused_variables)]
#![allow(unused_imports)]
// #![allow(dead_code)]

use pest::{
    error::{ErrorVariant, LineColLocation},
    Parser,
};
use std::{fs, include_str};

use ux_compiler::{Rule, UxParser};

pub mod ast;

fn pretty_error(positives: Vec<Rule>, line: usize, col: usize, input: &str) {
    let mut input_lines = input.lines();

    if let Some(data) = input_lines.nth(line - 1) {
        println!(
            r#"UnrecognizedToken "{:?}" at {} line, {} col"#,
            positives, line, col
        );
        println!();
        println!("{:4}|{}", line, data);
        println!("{:width$}^", "", width = col + 5);
    } else {
        println!("Empty line: {}", line);
    }
}

fn main() {

    let input = fs::read_to_string("tests/data/01.html").expect("cannot read file");

    match UxParser::parse(Rule::file, &input) {
        Ok(pairs) => {
            println!("Is Ok");
            // println!("Is Ok {:?}", pairs);
        }
        Err(err) => {
            // println!("{:?}", err);
            match err.variant {
                ErrorVariant::ParsingError {
                    positives,
                    negatives,
                } => match err.line_col {
                    LineColLocation::Pos((line, col)) => {
                        pretty_error(positives, line, col, &input);
                    }
                    _ => {
                        
                    }
                },
                _ => {}
            }
        }
    }

    // let file = UxParser::parse(Rule::file, &unparsed_file)
    //     .expect("unsuccessful parse") // unwrap the parse result
    //     .next()
    //     .unwrap(); // get and unwrap the `file` rule; never fails

    // let mut field_sum: f64 = 0.0;
    // let mut record_count: u64 = 0;
    // for record in file.into_inner() {
    //     match record.as_rule() {
    //         Rule::record => {
    //             record_count += 1;

    //             for field in record.into_inner() {
    //                 field_sum += field.as_str().parse::<f64>().unwrap();
    //             }
    //         }
    //         Rule::EOI => (),
    //         _ => unreachable!(),
    //     }
    // }

    // println!("Sum of fields: {}", field_sum);
    // println!("Number of records: {}", record_count);

    /////////////////////////////////////

    // let parse_result = Parser::parse(Rule::sum, "1773 + 1362").unwrap();
    // let tokens = parse_result.tokens();
    // for token in tokens {
    //     println!("{:?}", token);
    // }

    /////////////////////////////////////

    // let pair = Parser::parse(Rule::enclosed, "(..6472..) and more text")
    //     .unwrap()
    //     .next()
    //     .unwrap();
    // assert_eq!(pair.as_rule(), Rule::enclosed);
    // assert_eq!(pair.as_str(), "(..6472..)");
    // let inner_rules = pair.into_inner();
    // println!("{}", inner_rules); // --> [number(3, 7)]

    /////////////////////////////////////

    // let pairs = Parser::parse(Rule::sum, "1773 + 1362")
    //     .unwrap()
    //     .next()
    //     .unwrap()
    //     .into_inner();
    // let numbers = pairs
    //     .clone()
    //     .map(|pair| str::parse(pair.as_str()).unwrap())
    //     .collect::<Vec<i32>>();
    // assert_eq!(vec![1773, 1362], numbers);
    // for (found, expected) in pairs.zip(vec!["1773", "1362"]) {
    //     assert_eq!(Rule::number, found.as_rule());
    //     assert_eq!(expected, found.as_str());
    // }
}

#[test]
fn tests() {
    // assert!(UxParser::parse(Rule::normal_formal_parameters, r#"this.a, this.b"#).is_ok());
    // assert!(UxParser::parse(Rule::expression, "(22 * (44 + 66))").is_ok());

    // assert!(UxParser::parse(Rule::expression, r#"lx"#).is_ok());
    // assert!(UxParser::parse(Rule::expression, r#"-273.15"#).is_ok());
    // assert!(UxParser::parse(Rule::expression, r#""test drive unlimited""#).is_ok());
    // assert!(UxParser::parse(Rule::expression, r#"'s'"#).is_ok());
    // assert!(UxParser::parse(Rule::expression, r#"1"#).is_ok());
    // assert!(UxParser::parse(Rule::expression, r#"null"#).is_ok());
    // assert!(UxParser::parse(Rule::expression, r#"true"#).is_ok());

    // assert!(UxParser::parse(Rule::expression, r#"a = """sdasd asd """"#).is_ok());

    // let expr = UxParser::parse(Rule::testing, r#"1,
    // orElse: 2"#).unwrap();
    // assert_eq!(&format!("{:?}", expr), "=====");

    // assert!(UxParser::parse(Rule::literal, r#"s1"#).is_err());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CalculatorError {
    InputTooBig,
    OddNumber,
}

// fn pretty_error(token: Token, position: usize, input: &str) {
//     let mut input_lines = input.lines();

//     let tmp = &input[..position];
//     let lines = tmp.lines().enumerate();

//     if let Some((which, pointer)) = lines.last() {
//         if let Some(line) = input_lines.nth(which) {
//             println!(r#"UnrecognizedToken "{}" at {} line"#, token, which + 1);
//             println!();
//             println!("{:4}|{}", which + 1, line);
//             println!("{:width$}^", "", width = pointer.len() + 5);
//         } else {
//             println!("OOOPS!!! some wrong in pretty_error method");
//         }
//     }
// }
