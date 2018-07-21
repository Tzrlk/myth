//! The calculation that actually does the hard work.

use num::Integer;
use rand::thread_rng;
use rand::distributions::{ Range, IndependentSample };

use super::result::CalcResult;

/// This calculates which of the four results should occur given estimate and chaos scores.
pub fn calc(estimate: i32, chaos: i32, desire_yes: bool) -> CalcResult {

	// Make the three needed d10 rolls.
	let roll_1 = Range::new(1, 10).ind_sample(&mut thread_rng());
	let roll_2 = Range::new(1, 10).ind_sample(&mut thread_rng());
	let roll_3 = Range::new(1, 10).ind_sample(&mut thread_rng());

	// Calculate the result based on the rolls generated.
	return CalcResult {
		yes_result:   roll_is_yes(roll_1, roll_2, estimate, chaos, desire_yes),
		exceptional:  roll_is_exceptional(roll_1, roll_2, roll_3, chaos),
		random_event: roll_is_random_event(roll_1, roll_2, roll_3, chaos)
	};

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

fn roll_is_yes(roll_1: i32, roll_2: i32, estimate: i32, chaos: i32, desire_yes: bool) -> bool {
	return roll_1 + roll_2 + calc_estimate_mod(estimate) + calc_chaos_mod(chaos, desire_yes)
			>= 11;
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

	fn test_random_event(roll_1: i32, roll_2: i32, chaos_triggered: bool, expected: bool) {
		for chaos in 3..6 {
			let chaos_roll_lower = if chaos_triggered { 1 } else { chaos + 1 };
			let chaos_roll_upper = if chaos_triggered { chaos } else { 10 };
			for chaos_roll in chaos_roll_lower..chaos_roll_upper {
				assert_eq!(expected, roll_is_random_event(roll_1, roll_2, chaos_roll, chaos),
						"expected: {}, roll_1: {}, roll_2: {}, roll_3: {}, chaos: {}",
						expected, roll_1, roll_2, chaos_roll, chaos);
			}
		}
	}

	fn test_exceptional(roll_1: i32, roll_2: i32, chaos_triggered: bool, expected: bool) {
		for chaos in 3..6 {
			let chaos_roll_lower = if chaos_triggered { 1 } else { chaos + 1 };
			let chaos_roll_upper = if chaos_triggered { chaos } else { 10 };
			for chaos_roll in chaos_roll_lower..chaos_roll_upper {
				assert_eq!(expected, roll_is_exceptional(roll_1, roll_2, chaos_roll, chaos),
						"expected: {}, roll_1: {}, roll_2: {}, roll_3: {}, chaos: {}",
						expected, roll_1, roll_2, chaos_roll, chaos);
			}
		}
	}

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

			it "returns false for all roll results when current chaos > 3rd roll" {
				for roll_1 in 1..10 {
					for roll_2 in 1..10 {
						test_exceptional(roll_1, roll_2, false, false);
					}
				}
			}

			it "returns true for identical roll results when chaos < 3rd roll" {
				for roll_result in 1..10 {
					test_exceptional(roll_result, roll_result, true, true);
				}
			}

			it "returns true for odd roll results when chaos < 3rd roll" {
				for roll_1 in 1..10 {
					for roll_2 in 1..10 {
						let expected = roll_1 == roll_2 || roll_1.is_odd() && roll_2.is_odd();
						test_exceptional(roll_1, roll_2, true, expected);
					}
				}
			}

		}

		describe! roll_is_random_event {

			it "returns false for all roll results when current chaos > 3rd roll" {
				for roll_1 in 1..10 {
					for roll_2 in 1..10 {
						test_exceptional(roll_1, roll_2, false, false);
					}
				}
			}

			it "returns true for identical roll results when chaos < 3rd roll" {
				for roll_result in 1..10 {
					test_exceptional(roll_result, roll_result, true, true);
				}
			}

			it "returns true for even roll results when chaos < 3rd roll" {
				for roll_1 in 1..10 {
					for roll_2 in 1..10 {
						let expected = roll_1 == roll_2 || roll_1.is_even() && roll_2.is_even();
						test_exceptional(roll_1, roll_2, true, expected);
					}
				}
			}

		}

	}

}
