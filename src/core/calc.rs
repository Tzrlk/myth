//!

use estimate;
use bound;
use calc_result;
use rand::distributions::Range;

pub fn select(chaos: i32, prob: Estimate) {
	return select(chaos, prob.toInt());
}

pub fn select(chaos: i32, prob: i32) -> Bound {
	return table[prob][chaos];
}

pub fn calc(chaos: i32, prob: Estimate) {
	return calc(chaos, prob.toInt());
}

pub fn calc(chaos: i32, prob: i32) {

	let roll = Range::new(0, 100).ind_sample(rand::thread_rng());
	let bound = select(chaos, prob);

	if roll <= bound.lower  { return CalcResult::HellYes; }
	if roll <= bound.middle { return CalcResult::Yes;     }
	if roll <= bound.upper  { return CalcResult::No;      }
	                          return CalcResult::HellNo;

}
