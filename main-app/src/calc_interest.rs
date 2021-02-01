/*
*	Calculate simple interest and compound interest
*/

//'from' is safer than 'into'

pub fn calc_simple_interest(principal : f64, years : f32, rate : f32) -> f64 
{
	return ( principal * f64::from(years) * f64::from(rate) ) / 100.0;
}

pub fn calc_comp_interest(principal : f64, years : f32, rate : f32, t:i32 ) -> f64 
{
	return (principal* f64::powf(f64::from(1.0 + (f64::from(rate/100.0)/f64::from(t))), f64::from(years)))
			- principal;
}