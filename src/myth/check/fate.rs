//! The calculation that actually does the hard work.

use num::Integer;
use rand::thread_rng;
use rand::distributions::{ Range, IndependentSample };

use ::core::calc_result::CalcResult;

/// This calculates which of the four results should occur given estimate and chaos scores.
pub fn calc(estimate: i32, chaos: i32, desire_yes: bool) -> Option<CalcResult> {

	// Make the three needed d10 rolls.
	let roll_1 = Range::new(1, 10).ind_sample(&mut thread_rng());
	let roll_2 = Range::new(1, 10).ind_sample(&mut thread_rng());
	let roll_3 = Range::new(1, 10).ind_sample(&mut thread_rng());

	// Calculate the result based on the rolls generated.
	return calc_result(estimate, chaos, desire_yes, roll_1, roll_2, roll_3);

}

fn calc_result(
	estimate:   i32,
	chaos:      i32,
	desire_yes: bool,
	roll_1:     i32,
	roll_2:     i32,
	roll_3:     i32

) -> Option<CalcResult> {

	// Calculate the result of the check.
	let result = roll_1 + roll_2 + calc_estimate_mod(estimate) + calc_chaos_mod(chaos, desire_yes);
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

fn roll_is_exceptional(roll_1: i32, roll_2: i32, roll_3: i32, chaos: i32) -> bool {
	return ( roll_3 <= chaos ) && ( roll_1 == roll_2 || ( roll_1.is_odd() && roll_2.is_odd() ) );
}

fn roll_is_random_event(roll_1: i32, roll_2: i32, roll_3: i32, chaos: i32) -> bool {
	return ( roll_3 <= chaos ) && ( roll_1 == roll_2 || ( roll_1.is_even() && roll_2.is_even() ) );
}

#[cfg(test)]
mod tests {
	use super::*;

	describe! fate {

		describe! calc_result {

			it "produces an unexceptional result if chaos roll is greater than current chaos" {
				let result = calc_result(4, 3, true, 4, 7, 4).unwrap();
				assert_eq!(result.yes_result,   true);
				assert_eq!(result.exceptional,  false);
				assert_eq!(result.random_event, false);
			}

		}

		describe! calc_estimate_mod {

			it "returns +0 when estimate is fifty-fifty" {
				assert_eq!(calc_estimate_mod(5), 0)
			}

		}

		describe! calc_chaos_mod {

			it "returns +2 for low chaos when desired result is yes" {
				assert_eq!(calc_chaos_mod(1, true), 2);
				assert_eq!(calc_chaos_mod(2, true), 2);
				assert_eq!(calc_chaos_mod(3, true), 2);
			}

			it "returns -2 for low chaos when desired result is no" {
				assert_eq!(calc_chaos_mod(1, false), -2);
				assert_eq!(calc_chaos_mod(2, false), -2);
				assert_eq!(calc_chaos_mod(3, false), -2);
			}

			it "returns +0 for middle chaos, regardless of desired result" {
				assert_eq!(calc_chaos_mod(4, true),  0);
				assert_eq!(calc_chaos_mod(5, true),  0);
				assert_eq!(calc_chaos_mod(4, false), 0);
				assert_eq!(calc_chaos_mod(5, false), 0);
			}

			it "returns -2 for high chaos when desired result is yes" {
				assert_eq!(calc_chaos_mod(6, true), -2);
				assert_eq!(calc_chaos_mod(7, true), -2);
				assert_eq!(calc_chaos_mod(8, true), -2);
			}

			it "returns +2 for high chaos when desired result is no" {
				assert_eq!(calc_chaos_mod(6, false), 2);
				assert_eq!(calc_chaos_mod(7, false), 2);
				assert_eq!(calc_chaos_mod(8, false), 2);
			}

		}

		describe! roll_is_exceptional {

			it "returns false when chaos above 3rd roll" {
				for chaos in 3..6 {
					let roll_1 = Range::new(1, 10).ind_sample(&mut thread_rng());
					let roll_2 = Range::new(1, 10).ind_sample(&mut thread_rng());
					for chaos_roll in (chaos + 1)..10 {
						assert!(!roll_is_exceptional(roll_1, roll_2, chaos_roll, chaos));
					}
				}
			}

		}

		describe! roll_is_random_event {

			it "returns false when chaos above 3rd roll" {
				for chaos in 3..6 {
					let roll_1 = Range::new(1, 10).ind_sample(&mut thread_rng());
					let roll_2 = Range::new(1, 10).ind_sample(&mut thread_rng());
					for chaos_roll in (chaos + 1)..10 {
						assert!(!roll_is_random_event(roll_1, roll_2, chaos_roll, chaos));
					};
				}
			}

		}

	}

}
