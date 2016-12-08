//! build scripts

extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{ BufWriter, Write };
use std::path::Path;

fn main() {
	let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
	let mut file = BufWriter::new(File::create(&path).unwrap());

	write!(&mut file, "static KEYWORDS: phf::Map<&'static str, Keyword> = ").unwrap();


/*
	val table: Map<BaseProb, Array<P>> = mapOf(
	HasToBe        to arrayOf( P(16,80,97), P(16,85,97), P(18,90,99), P(19,95,100), P(19,95,100), P(20,100,101), P(20,100,101), P(26,130,131), P(26,145,146) ),
	SureThing      to arrayOf( P(11,55,92), P(13,65,94), P(16,80,97), P(16,85,97),  P(18,90,99),  P(19,95,100),  P(19,95,100),  P(22,110,111), P(25,125,126) ),
	NearSureThing  to arrayOf( P(10,50,91), P(11,55,92), P(15,75,96), P(16,80,97),  P(18,90,99),  P(19,95,100),  P(19,95,100),  P(20,100,101), P(23,115,116) ),
	VeryLikely     to arrayOf( P(9,45,90),  P(10,50,91), P(13,65,94), P(15,75,96),  P(16,85,97),  P(18,90,99),   P(19,95,100),  P(19,95,100),  P(21,105,106) ),
	Likely         to arrayOf( P(5,25,86),  P(7,35,88),  P(10,50,91), P(11,55,92),  P(15,75,96),  P(16,85,97),   P(18,90,99),   P(19,95,100),  P(20,100,101) ),
	SomewhatLikely to arrayOf( P(4,10,85),  P(5,25,86),  P(9,45,90),  P(10,50,91),  P(13,65,94),  P(16,80,97),   P(16,85,97),   P(18,90,99),   P(19,95,100) ),
	FiftyFifty     to arrayOf( P(2,10,83),  P(3,15,84),  P(5,25,86),  P(7,35,88),   P(10,50,91),  P(13,65,94),   P(15,75,96),   P(16,85,97),   P(19,95,100) ),
	Unlikely       to arrayOf( P(1,5,82),   P(2,10,83),  P(3,15,84),  P(4,20,85),   P(7,35,88),   P(10,50, 91),  P(11,55,92),   P(15,75,96),   P(18,90,99) ),
	VeryUnlikely   to arrayOf( P(1,5,82),   P(1,5,82),   P(2,10,83),  P(3,15,84),   P(5,25,86),   P(9,45,90),    P(10,50,91),   P(13,65,94),   P(16,85,97) ),
	NoWay          to arrayOf( P(0,0,81),   P(1,5,82),   P(1,5,82),   P(2,10,83),   P(3,15,84),   P(5,25,86),    P(7,35,88),    P(10,50,91),   P(15,75,96) ),
	Impossible     to arrayOf( P(0,-20,77), P(0,0,81),   P(0,0,81),   P(1,5,82),    P(1,5,82),    P(2,10,83),    P(3,15,84),    P(5,25,86),    P(10,50,91) )
	)
*/

	phf_codegen::Map::new()
		.entry("Estimate::HasToBe", "&[ Bound(16,80,97), Bound(16,85,97), Bound(18,90,99), Bound(19,95,100), Bound(19,95,100), Bound(20,100,101), Bound(20,100,101), Bound(26,130,131), Bound(26,145,146) ]")
		.build(&mut file)
		.unwrap();

	write!(&mut file, ";\n").unwrap();

}
