//! The calculation that actually does the hard work.

use num::Integer;
use rand::thread_rng;
use rand::distributions::{ Range, IndependentSample };

use ::core::calc_result::CalcResult;

/// This calculates which of the four results should occur given estimate and chaos scores.
pub fn calc(estimate: i32, chaos: i32, desired_result: bool) -> Option<CalcResult> {

	// Make the three needed d10 rolls.
	let roll_1 = Range::new(1, 10).ind_sample(&mut thread_rng());
	let roll_2 = Range::new(1, 10).ind_sample(&mut thread_rng());
	let roll_3 = Range::new(1, 10).ind_sample(&mut thread_rng());

	// Calculate the result of the check.
	let result = roll_1 + roll_2 + calc_estimate_mod(estimate) + calc_chaos_mod(chaos, desired_result);
	let is_yes = result >= 11;

	// If the chaos die is above the chaos factor, don't bother calculating chaos cases and just
	// return early.
	if roll_3 > chaos {
		return Some(CalcResult {
			yes_result:   is_yes,
			exceptional:  false,
			random_event: false
		});
	}

	// Calculate the chaos cases.
	let exceptional  = ( roll_1.is_odd()  && roll_2.is_odd()  ) || ( roll_1 == roll_2 );
	let random_event = ( roll_1.is_even() && roll_2.is_even() ) || ( roll_1 == roll_2 );

	// Return the appropriate result.
	return Some(CalcResult {
		yes_result: is_yes,
		exceptional,
		random_event
	});

}

fn calc_estimate_mod(estimate: i32) -> i32 {
	return estimate - 5;
}

fn calc_chaos_mod(chaos: i32, desired: bool) -> i32 {
	return if chaos < 4 { // low chaos
		if desired {  2 } // +2 for desired 'yes' result
		      else { -2 } // -2 for desired 'no' result
	} else if chaos > 5 { // high chaos
		if desired { -2 } // -2 against desired 'yes' result
		      else {  2 } // +2 against desired 'no' result
	} else {
		0 // no modification
	};
}
