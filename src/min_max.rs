/*
FInd the min and max of 3 numbers
*/
use std::env;
use std::process::exit;

use std::result::Result;

fn num_parse(nargs : &str) -> i64 {
	let presult : Result<i64, _> = nargs.parse();
	match presult {
		Ok(s) => return s,
		Err(e) => { println!("Invalid number sent in \'{}\'. Exception : {}", nargs, e); }
	}
	return 0;
}

pub fn min_max() {
	let args : Vec<String> = env::args().collect();
		//args[0] is the task name
	if args.len() != 4
	{ println!("Please input 3 numbers only to compare"); exit(0);}
 
	let num_parse_lambda = | nargs : &str | -> i64 {
		match nargs.parse() {
			Ok(s) => s,
			Err(e) => { println!("Invalid number sent in lambda \'{}\'. Exception : {}", nargs, e); 0 }
		}
	};

		let num1 : i64 = num_parse(&args[1]); //.parse().unwrap();
		let num2 : i64 = num_parse_lambda(&args[2]);
		let num3 : i64 = args[3].parse().unwrap();

		if num1 == 0 || num2 == 0 || num3 == 0 { exit(1); }

		let _ = if num1 < num2 && num3 < num2 { num2 } 
					else if num2 < num1 && num3 < num1 { num1 }
					else { num3 };
		let _ = if num1 > num2 && num3 > num2 { num2 }
					else if num2 > num3 && num1 > num3 { num3 }
					else { num1 };
	
	//println!("The Max # is [{}] and the Min # is [{}]", max, min);
}