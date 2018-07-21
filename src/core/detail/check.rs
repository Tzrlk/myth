//!

use rand::distributions::{ Range, IndependentSample };
use rand::thread_rng;

use super::CheckResult;
use super::super::Chaos;

pub fn check(chaos: Chaos) -> CheckResult {

	let roll_1 = Range::new(1, 10).ind_sample(&mut thread_rng());
	let roll_2 = Range::new(1, 10).ind_sample(&mut thread_rng());

	return CheckResult::from(roll_1 + roll_2 + chaos.to_mod())

}
