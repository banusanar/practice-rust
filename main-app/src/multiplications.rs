/*
*  Write a program that prints a multiplication table for a given number and the number of rows in
*  the table. For example, for a number 5 and rows = 3, the output should be:
*  5 x 1 = 5
*  5 x 2 = 10
*  5 x 3 = 15
*
*/

pub fn multiplication_table(multiples: i32, rows: i32) {
    for factor in 1..(rows + 1) {
        println!("{} x {} = {}", multiples, factor, multiples * factor)
    }
}
