#![allow(dead_code)]
extern crate argparse;
extern crate expressions;
//use expressions::math::time_per_floor;
use argparse::ArgumentParser;
use argparse::Store;

mod binary;
mod calc_interest;
mod marks;
mod min_max;
mod multiplications;

use std::convert::TryFrom;
use std::env;
use std::process::exit;

fn num_parse(nargs: &str) -> i64 {
    let presult: Result<i64, _> = nargs.parse();
    match presult {
        Ok(s) => return s,
        Err(e) => {
            println!("Invalid number sent in \'{}\'. Exception : {}", nargs, e);
        }
    }
    return 0;
}

fn min_max_main() {
    let args: Vec<String> = env::args().collect();
    //args[0] is the task name
    if args.len() != 4 {
        println!("Please input 3 numbers only to compare");
        exit(0);
    }

    let num_parse_lambda = |nargs: &str| -> i64 {
        match nargs.parse() {
            Ok(s) => s,
            Err(e) => {
                println!(
                    "Invalid number sent in lambda \'{}\'. Exception : {}",
                    nargs, e
                );
                0
            }
        }
    };

    let num1: i64 = num_parse(&args[1]); //.parse().unwrap();
    let num2: i64 = num_parse_lambda(&args[2]);
    let num3: i64 = args[3].parse().unwrap();
    min_max::min_max(num1, num2, num3);
}

fn calc_interest_main() {
    let mut principal: f64 = 0.0;
    let mut time: f32 = 1.0; //time in years
    let mut rate: f32 = 0.0;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Calculate interest");
        ap.refer(&mut principal).add_option(
            &["-p", "--principal"],
            Store,
            "principal to calculate interest",
        );
        ap.refer(&mut time)
            .add_option(&["-t", "--time"], Store, "Time in years for loan");
        ap.refer(&mut rate)
            .add_option(&["-r", "--rate"], Store, "Interate rate");
        ap.parse_args_or_exit();
    }

    let si = calc_interest::calc_simple_interest(principal, time, rate);
    let ci = calc_interest::calc_comp_interest(principal, time, rate, 2);

    println!("Simple interest = {} and Compound Interest = {}", si, ci);
}

fn marks_main() {
    marks::process_marks_sheet("resources/student_marks_sheet.txt");
}

fn multiplication_main() {
    let args: Vec<String> = env::args().collect();
    //args[0] is the task name
    if args.len() != 3 {
        println!("Please input 2 numbers to start creating table");
        exit(0);
    }

    let num_parse_lambda = |nargs: &str| -> i32 {
        match nargs.parse() {
            Ok(s) => s,
            Err(e) => {
                println!(
                    "Invalid number sent in lambda \'{}\'. Exception : {}",
                    nargs, e
                );
                0
            }
        }
    };

    let multiples: i32 = num_parse_lambda(&args[1]);
    let rows: i32 = num_parse_lambda(&args[2]);

    multiplications::multiplication_table(multiples, rows);
}

fn binary_main() {
    let args: Vec<String> = env::args().collect();
    //args[0] is the task name
    if args.len() != 2 {
        println!("Please input number to convert to binary");
        exit(0);
    }

    let inputb = u16::try_from(num_parse(&args[1])).unwrap();
    let input8 = u64::try_from(num_parse(&args[1])).unwrap();
    binary::convert_to_binary(inputb);
    binary::convert_to_octal(input8);
}

fn topfloor_main() {
    let args: Vec<String> = env::args().collect();
    //args[0] is the task name
    if args.len() != 3 {
        println!("Please input top floor and the height of each floor");
        exit(0);
    }

    let topfloor = i8::try_from(num_parse(&args[1])).unwrap();
    let height = i8::try_from(num_parse(&args[2])).unwrap();

    for x in (0..topfloor).rev() {
        let timet = expressions::math::time_per_floor(topfloor, height, x);
        println!("floor {} : time taken to reach {}", x, timet);
    }
}

fn calc_main() {
    let args: Vec<String> = env::args().collect();
    //args[0] is the task name
    if args.len() != 4 {
        println!("Please input lhs and rhs and operand");
        exit(0);
    }

    let lhs = i32::try_from(num_parse(&args[1])).unwrap();
    let rhs = i32::try_from(num_parse(&args[2])).unwrap();
    let operand = String::from(&args[3]);

    let res = expressions::math::calc_expression::<i32>(lhs, rhs, &operand);
    println!("Result: {}", res)
}

fn main() {
    expressions::math::is_a_prime(7i64);
}
