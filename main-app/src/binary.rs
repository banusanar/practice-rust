/*
* Write a program that shows the binary equivalent of a given positive number between 0 to 255.
*/
use std::vec::Vec;
use std::char;
use std::convert::TryFrom;

pub fn convert_to_binary(input : u16) -> String {
	let mut digits :Vec<char> = Vec::new();
	let mut temp = input; 

	while temp > 1 {
		digits.push(char::from_digit(u32::from(temp%2), 10).unwrap());	
		temp = temp / 2;
	}

	digits.push(char::from_digit(u32::from(temp), 10).unwrap());
	digits.reverse();
	digits.iter().collect::<String>()
	/*print!("0x");
	for x in digits.iter()
		{ print!("{}", x); }
	println!(); */
}

pub fn convert_to_octal(input : u64) -> String {
	let mut temp = input;
	let mut digits :Vec<char> = Vec::new();
	while temp > 7 {
		digits.push(char::from_digit(u32::try_from(temp%8).unwrap(), 10).unwrap());	
		temp = temp / 8;	
	}
	digits.push(char::from_digit(u32::try_from(temp).unwrap(), 10).unwrap());
	digits.reverse();
	digits.iter().collect::<String>()

	/*print!("0x");
	for x in digits.iter()
		{ print!("{}", x); }
	println!();*/
}

#[test]
fn t_binary() {
	assert_eq!("10", convert_to_binary(2));
	assert_eq!("1", convert_to_binary(1));
	assert_eq!("1111", convert_to_binary(15));
	assert_eq!("101010", convert_to_binary(42));

	assert_eq!("0", convert_to_octal(0));
	assert_eq!("7", convert_to_octal(7));
	assert_eq!("10", convert_to_octal(8));
	assert_eq!("11", convert_to_octal(9));
	assert_eq!("100", convert_to_octal(64));
}