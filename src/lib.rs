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

impl core::str::FromStr for GradientType {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input.to_ascii_lowercase().as_str() {
			"l" | "linear" => Ok(Self::Linear),
			"r" | "radial" => Ok(Self::Radial),
			_ => Err(()),
		}
	}
}

/// returns an `Err` if any color contains `"`, regardless if it's escaped or not.
///
/// this "syntax validation" is done for security reasons (prevent code injection).
pub fn generate(t: GradientType, colors: Vec<String>) -> Result<String, ()> {
	use fmt::Write as _;

	if colors.iter().any(|x| x.contains('"')) {
		return Err(());
	}

	let color_count = colors.len();

	let body = colors
		.into_iter()
		.enumerate()
		.map(|(i, c)| {
			// `+ 16` is temporary. to-do: use a better estimation
			let mut s = String::with_capacity(c.len() + 16);
			let _ = write!(
				s,
				"<stop offset=\"{}%\" stop-color=\"{}\"/>",
				i * 100 / (color_count - i.min(1)),
				c
			);
			s
		})
		.collect::<Vec<String>>()
		.join("");

	// `+ 64` is temporary. to-do: use a better estimation
	let mut out = String::with_capacity(body.len() + 64);

	let _ = write!(
		out,
		// should this have a viewBox?
		"\
		<?xml version=\"1.0\" encoding=\"utf-8\"?>\
		<svg xmlns=\"http://www.w3.org/2000/svg\">\
		<defs>\
		<{}Gradient id=\"g\"{}>\
		{}\
		</{}Gradient>\
		</defs>\
		<rect width=\"100%\" height=\"100%\" fill=\"url('#g')\"/>\
		</svg>\
		",
		t,
		match t {
			GradientType::Linear => " gradientTransform=\"rotate(90)\"",
			GradientType::Radial => "",
		},
		body,
		t
	);

	Ok(out)
}
