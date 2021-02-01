/*
FInd the min and max of 3 numbers
*/
use std::process::exit;
pub fn min_max(num1 :i64, num2:i64, num3:i64) {


		if num1 == 0 || num2 == 0 || num3 == 0 { exit(1); }

		let _ = if num1 < num2 && num3 < num2 { num2 } 
					else if num2 < num1 && num3 < num1 { num1 }
					else { num3 };
		let _ = if num1 > num2 && num3 > num2 { num2 }
					else if num2 > num3 && num1 > num3 { num3 }
					else { num1 };
	
	//println!("The Max # is [{}] and the Min # is [{}]", max, min);
}