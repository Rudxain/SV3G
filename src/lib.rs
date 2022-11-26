#![deny(clippy::unwrap_used)]
#![forbid(clippy::exit)]

use core::fmt;

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

	let size = colors.len();

	let part = colors
		.into_iter()
		.enumerate()
		.map(|(i, c)| {
			let mut s = String::with_capacity(c.len() + 16);
			let _ = write!(
				s,
				"<stop offset=\"{}%\" stop-color=\"{}\"/>",
				i * 100 / (size - i.min(1)),
				c
			);
			s
		})
		.collect::<Vec<String>>()
		.join("");

	let mut out = String::with_capacity(part.len() + 64);

	let _ = write!(
		out,
		// this is so ugly
		"{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
		"<?xml version=\"1.0\" encoding=\"utf-8\"?>",
		// should this have a viewBox?
		"<svg xmlns=\"http://www.w3.org/2000/svg\">",
		"<defs>",
		"<",
		t,
		"Gradient id=\"g\"",
		match t {
			GradientType::Linear => " gradientTransform=\"rotate(90)\"",
			GradientType::Radial => "",
		},
		">",
		part,
		"</",
		t,
		"Gradient>",
		"</defs>",
		"<rect width=\"100%\" height=\"100%\" fill=\"url('#g')\"/>",
		"</svg>"
	);

	out
}
