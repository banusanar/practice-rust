/*
* A building has 10 floors with a floor height of 3 meters each. A ball is dropped from the top of
* the building. Find the time taken by the ball to reach each floor. (Use the formula s = ut+(1/2)at^2
* where u and a are the initial velocity in m/sec (= 0) and acceleration in m/sec^2 (= 9.8 m/s^2)). 
*/

extern crate num_traits;

use math::num_traits::AsPrimitive;

pub fn time_per_floor( topfloor : i8, fheight : i8, inpfloor : i8) -> f32
{
	let dist = f32::from((topfloor - inpfloor) * fheight);
	let acc = f32::from(9.8);
	//s = 1/2 * a * t ^2; so
	//t = sq_root of (2s/a);
	return ((2.0 * dist) / acc).sqrt();
}

/*
*Write a program, which takes two integer operands and one operator from the user, performs
*the operation and then prints the result. (Consider the operators +,-,*, /, % ) 
*/

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;
/*
fn calc_gen_add<T>(lhs : T, rhs: T) -> T
where T : Add<Output = T> + Sub<Output = T>,
	{	lhs + rhs	}
	
fn calc_gen_sub<T : Sub<Output = T>>(lhs : T, rhs: T) -> T
	{	lhs - rhs	}
	*/

pub fn calc_expression<T> (lhs :T , rhs :T, operand :&str) -> T
where T : AsPrimitive<T> + 
	Add<Output = T> + 
	Sub<Output = T> + 
	Mul<Output = T> + 
	Div<Output = T> +
	Rem<Output = T>
{
	match operand 
	{
		"+" => lhs + rhs,
		"-" => lhs - rhs,
		"*" => lhs * rhs,
		"/" => lhs / rhs,
		"%" => lhs % rhs,
		_	=> lhs
	}
}
