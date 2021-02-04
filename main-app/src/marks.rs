//#![feature(exhaustive_integer_patterns)]
//#![feature(exclusive_range_pattern)]

/*
* Write program that declares Class awarded for a given percentage of marks, where mark
* <40%= Failed, 40% to <60% = Second class, 60% to <70%=First class, >= 70% = Distinction.
* Read percentage from standard input.
*
*/

use std::num::ParseIntError;
use std::result::Result;
use std::fs;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Student {
	name: String,
	marks:i32,
	class: String
}

impl fmt::Display for Student {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) ->fmt::Result {
		write!(f, "(Name: {}, Marks: {}, Class: {})", self.name, self.marks, self.class)
	}
}

impl FromStr for Student {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let tokens: Vec<&str> = s.split(':').collect();
		println!("tokens {:?}", tokens);	//This uses the #Debug print feature
		let l_name = tokens[0];
		let l_marks = tokens[1].trim().parse::<i32>()?;
		Ok(Student { name : l_name.to_string(), marks : l_marks, class : String::from("unknown")})
	}
}	


pub fn process_marks_sheet(filename :&str) -> Vec<Student>
{
	let contents : Vec< Result<Student, <Student as FromStr>::Err> > = 
				fs::read_to_string(filename)
					.expect("Read error from file")
					.lines()
					.map(|x| x.parse())
					.collect();
	let mut students : Vec< Student> = Vec::new();
	for stud in contents
	{
		match stud {
			Ok(mut s) => {	//mutable can be done on a non-mutable top level element
							// const vector containing const elements is no longer true
				s.class = get_class_for_marks(s.marks);
				println!("{}",s);	//This uses the fmt::display implemented for Student
				students.push(s);
				},
			Err(e) => println!("{}", e) 
		}
	}
	students
}

fn get_class_for_marks(marks :i32) -> String {
	match marks {
		x if x<40 => String::from("Fail"),
		x if x>=40 && x<60 => String::from("Second class"),
		x if x>=60 && x<70 => String::from("First class"),
		x if x>=70 && x<=100 => String::from("Distinction"),
		_ => String::from("Invalid marks")
	}
}

#[test]
fn t_marks() {
	let students = process_marks_sheet("resources/student_marks_sheet.txt");
	for stud in students {
		match stud.marks {
			x if x<40 => assert_eq!(stud.class, "Fail"), 
			x if x>=40 && x<60 => assert_eq!(stud.class,"Second class"),
			x if x>=60 && x<70 => assert_eq!(stud.class,"First class"),
			x if x>=70 && x<=100 => assert_eq!(stud.class,"Distinction"),
			_ => assert_eq!(stud.class,"Invalid marks")
		}
	}
}