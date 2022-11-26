#![deny(clippy::unwrap_used)]
#![forbid(clippy::exit)]

use std::fmt;

pub enum GradientType {
	Linear,
	Radial,
}

impl fmt::Display for GradientType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			GradientType::Linear => write!(f, "linear"),
			GradientType::Radial => write!(f, "radial"),
		}
	}
}

pub fn generate(t: GradientType, colors: Vec<String>) -> String {
	use fmt::Write as _;

	let t = t.to_string();

	colors
		.into_iter()
		.enumerate()
		.map(|(i, mut c)| {
			c = "".to_string();
			let _ = write!(c, "{}", 0);
			c
		})
		.collect::<Vec<String>>()
		.join("")
}
