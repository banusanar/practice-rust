
extern crate argparse;
use argparse::ArgumentParser;
use argparse::Store;


mod calc_interest;
mod min_max;
mod marks;

#[allow(dead_code)]
fn min_max_main() {
	min_max::min_max();
}

#[allow(dead_code)]
fn calc_interest_main() {

	let mut principal :f64 = 0.0;
	let mut time :f32 = 1.0;	//time in years
	let mut rate :f32 = 0.0;

	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Calculate interest");
		ap.refer(&mut principal)
		.add_option(&["-p", "--principal"], Store, "principal to calculate interest");
		ap.refer(&mut time)
		.add_option(&["-t", "--time"], Store, "Time in years for loan");
		ap.refer(&mut rate)
		.add_option(&["-r", "--rate"], Store, "Interate rate");
		ap.parse_args_or_exit();
	}

	let si = calc_interest::calc_simple_interest(principal, time, rate);
	let ci = calc_interest::calc_comp_interest(principal, time, rate, 2);

	println!("Simple interest = {} and Compound Interest = {}", si, ci);
}

fn main() {
	
	marks::process_marks_sheet("resources/student_marks_sheet.txt");
}
