extern crate leetcode;
//use expressions::math::calc_expression;

#[test]
fn boats_num_rescue_boats_test()
{
	assert_eq!(leetcode::boats::num_rescue_boats(vec![3u8,2u8,3u8,4u8], 5u8), 3usize);
}