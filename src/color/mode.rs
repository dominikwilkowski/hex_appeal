#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
	DoubleA,
	TripleA,
}

impl Mode {
	pub fn ratio_large_text(&self, ratio: f64) -> bool {
		match self {
			Mode::DoubleA => ratio >= 4.5,
			Mode::TripleA => ratio >= 7.0,
		}
	}

	pub fn ratio_small_text(&self, ratio: f64) -> bool {
		match self {
			Mode::DoubleA => ratio >= 3.0,
			Mode::TripleA => ratio >= 4.5,
		}
	}
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Mode::DoubleA => write!(f, "AA"),
			Mode::TripleA => write!(f, "AAA"),
		}
	}
}
