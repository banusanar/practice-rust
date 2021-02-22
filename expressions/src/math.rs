/*
* A building has 10 floors with a floor height of 3 meters each. A ball is dropped from the top of
* the building. Find the time taken by the ball to reach each floor. (Use the formula s = ut+(1/2)at^2
* where u and a are the initial velocity in m/sec (= 0) and acceleration in m/sec^2 (= 9.8 m/s^2)).
*/

extern crate num_traits;
use std::convert::TryFrom;
use std::vec::Vec;

//use math::num_traits::AsPrimitive;

pub fn time_per_floor(topfloor: i8, fheight: i8, inpfloor: i8) -> f32 {
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
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;
/*
fn calc_gen_add<T>(lhs : T, rhs: T) -> T
where T : Add<Output = T> + Sub<Output = T>,
    {	lhs + rhs	}

fn calc_gen_sub<T : Sub<Output = T>>(lhs : T, rhs: T) -> T
    {	lhs - rhs	}
    */

pub fn calc_expression<T>(lhs: T, rhs: T, operand: &str) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
    match operand {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        "%" => lhs % rhs,
        _ => lhs,
    }
}

pub fn is_a_prime(lhs: i64) -> bool {
    let mut mult: i64 = 2;

    loop {
        if lhs % mult == 0i64 {
            return false;
        }
        mult = mult + 1i64;
        if mult > (lhs / 2) {
            break;
        }
    }
    return true;
}

pub fn sum_of_digits(inp : u64) -> i8 {
    let mut sum:i8 = 0;
    let mut temp = inp;
    while temp > 0 {
        sum = sum + i8::try_from(temp % 10u64).unwrap();
        temp = temp / 10u64;
    }
    return sum;
}

pub fn gen_fibonnaci(inp : usize) -> Vec<u64> {
    let mut fseries : Vec<u64> = Vec::with_capacity(inp);
    fseries.push(0); fseries.push(1);
    let mut idx = 2usize;
        
    while idx < inp {
        fseries.push(fseries[idx - 1usize] + fseries[idx - 2usize]);
        idx = idx + 1usize;
    }
    return fseries;
}

