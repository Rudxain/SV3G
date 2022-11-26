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
	let size = colors.len();

	colors
		.into_iter()
		.enumerate()
		.map(|(i, c)| {
			let s = String::with_capacity(size * 5);
			let _ = write!(s, "<stop offset=\"{}%\" stop-color=\"{}\"/>", i / (size - i.max(1)) * 100, c);
			s
		})
		.collect::<Vec<String>>()
		.join("")
}
