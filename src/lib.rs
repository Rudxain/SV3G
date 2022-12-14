#![deny(clippy::unwrap_used)]
#![forbid(clippy::exit)]

use core::fmt;

const LINE: &str = "linear";
const RAD: &str = "radial";
/*
// is this really a good idea?
// I just want to validate colors at compile-time, to avoid `unwrap`
/// 32bit color value
pub struct Color32 {
	/// red
	r: u8,
	/// green
	g: u8,
	/// blue
	b: u8,
	/// alpha
	a: u8
}

impl Color32 {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }
}

impl fmt::Display for Color32 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
	}
}
*/

pub enum GradientType {
	Linear,
	Radial,
	//Conic
}

impl fmt::Display for GradientType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			GradientType::Linear => write!(f, "{}", LINE),
			GradientType::Radial => write!(f, "{}", RAD),
		}
	}
}

impl core::str::FromStr for GradientType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"l" | LINE => Ok(Self::Linear),
			"r" | RAD => Ok(Self::Radial),
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
pub struct ColorQuotes;

impl fmt::Display for ColorQuotes {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "CSS colors must not contain quotes")
	}
}

// to-do: use `core` when stable
impl std::error::Error for ColorQuotes {}

pub fn read_u8() -> Result<u8, ColorQuotes> {
	Err(ColorQuotes)
}

/// returns an `Err` if any color contains `"`, regardless if it's escaped or not.
///
/// this syntax validation is done for security reasons (prevent code injection).
pub fn generate(t: GradientType, colors: Vec<String>) -> Result<String, ColorQuotes> {
	use fmt::Write as _;

	if colors.iter().any(|x| x.contains('"')) {
		return Err(ColorQuotes);
	}

	// avoid borrow
	let color_count = colors.len();

	let body = colors
		.into_iter()
		.enumerate()
		.map(|(i, c)| {
			// `+ 16` is temporary. to-do: use a better estimation
			let mut s = String::with_capacity(c.len() + 16);
			let _ = write!(
				s,
				// is the percent really necessary?
				// can we avoid the `* 100`?
				"<stop offset=\"{}%\" stop-color=\"{}\"/>",
				// this double conversion is annoying for me
				// I'm using floats just to div and print, feels a lil hacky
				i.saturating_mul(100) as f64 / (color_count - i.min(1)) as f64,
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
