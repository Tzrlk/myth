//! The calculation that actually does the hard work.

use rand::thread_rng;
use rand::distributions::{ Range, IndependentSample };

use super::calc_result::CalcResult;
use super::bounds::TABLE;

/// This calculates which of the four results should occur given estimate and chaos scores.
pub fn calc(estimate: i32, chaos: i32) -> Option<CalcResult> {

	let roll = Range::new(1, 101).ind_sample(&mut thread_rng());
	let bound = TABLE.as_ref()[estimate as usize][chaos as usize];

	if roll <= bound[0] { return Some(CalcResult::HellYes); }
	if roll <= bound[1] { return Some(CalcResult::Yes);     }
	if roll <= bound[2] { return Some(CalcResult::No);      }
	                      return Some(CalcResult::HellNo);

}
