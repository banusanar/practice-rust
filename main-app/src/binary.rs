/*
* Write a program that shows the binary equivalent of a given positive number between 0 to 255.
*/
use std::vec::Vec;

pub fn convert_to_binary(input : u16) {
	let mut digits :Vec<u16> = Vec::new();
	let mut temp = input; 

	while temp > 1 {
		digits.push(temp%2);	
		temp = temp / 2;
	}

	digits.push(temp);digits.reverse();
	print!("0x");
	for x in digits.iter()
		{ print!("{}", x); }
	println!();
}

pub fn convert_to_octal(input : u64) {
	let mut temp = input;
	let mut digits :Vec<u64> = Vec::new();
	while temp > 7 {
		digits.push(temp%8);
		temp = temp / 8;	
	}
	digits.push(temp);
	digits.reverse();
	print!("0x");
	for x in digits.iter()
		{ print!("{}", x); }
	println!();
}