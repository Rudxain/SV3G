#![warn(
	unused,
	future_incompatible,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string
)]
#![allow(clippy::uninlined_format_args)]
#![deny(clippy::unwrap_used, clippy::float_arithmetic)]
#![forbid(
	unsafe_code,
	clippy::exit,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]

use core::{fmt, str::FromStr};

#[allow(clippy::cast_precision_loss, clippy::float_arithmetic)]
/*const */
fn div_usize_as_f64(n: usize, d: usize) -> f64 {
	n as f64 / d as f64
}

const LINE: &str = "linear";
const RAD: &str = "radial";

/// just like `Path` is internally an `OsStr`,
/// this `struct` is just a `String`,
/// but guaranteed to be a subset, such that it matches the syntax of CSS colors
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CSSColor(String);

impl CSSColor {
	/// # `CSSColor::new`
	///
	/// Directly wraps a `String` as a `CSSColor`, only if it has valid syntax.
	///
	/// The current implementation only checks for the presence of quotes,
	/// so it it quite permissive.
	/// More errors and stricter validation will be added later.
	///
	/// ## Errors
	///
	/// `ColorQuotes`: happens when the string contains 1 or more double quotes (")
	pub fn new(s: String) -> Result<Self, ColorQuotes> {
		if s.contains('"') {
			return Err(ColorQuotes);
		}
		Ok(Self(s))
	}
}

impl fmt::Display for CSSColor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum GradientType {
	Linear,
	Radial,
	//Conic,
}

impl fmt::Display for GradientType {
	// I wonder why Rust needs boilerplate here
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Linear => write!(f, "{}", LINE),
			Self::Radial => write!(f, "{}", RAD),
		}
	}
}
// I hope rustc optimizes both of these conversions into no-ops
impl FromStr for GradientType {
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
//core
impl std::error::Error for ColorQuotes {}
// https://github.com/rust-lang/rustfmt/issues/5320#issuecomment-1363417363

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn generate(t: &GradientType, colors: Vec<CSSColor>) -> String {
	use fmt::Write as _;

	let colors: Vec<String> = colors.into_iter().map(|c| c.to_string()).collect();

	// avoid borrow
	let color_count = colors.len();

	let body: String = colors
		.into_iter()
		.enumerate()
		.map(|(i, c)| {
			// to-do: use a better estimation than `0x10`
			let mut s = String::with_capacity(c.len() + 0x10);
			let _ = write!(
				s,
				"<stop offset=\"{}\" stop-color=\"{}\"/>",
				div_usize_as_f64(i, color_count - i.min(1)),
				c
			);
			s
		})
		.collect();

	// to-do: use a better estimation than `0x40`
	let mut out = String::with_capacity(body.len() + 0x40);

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
		<rect width=\"100%\" height=\"100%\" fill=\"url(#g)\"/>\
		</svg>\
		",
		t,
		if *t == GradientType::Linear {
			" gradientTransform=\"rotate(90)\""
		} else {
			""
		},
		body,
		t
	);

	out
}

#[cfg(test)]
#[test]
fn identical_enum_str_gradient_conversion() {
	assert_eq!(
		GradientType::from_str(LINE).expect("").to_string(),
		LINE.to_string()
	);
	assert_eq!(
		GradientType::from_str(RAD).expect("").to_string(),
		RAD.to_string()
	);
	assert_eq!(
		GradientType::from_str(&GradientType::Linear.to_string()).expect(""),
		GradientType::Linear
	);
	assert_eq!(
		GradientType::from_str(&GradientType::Radial.to_string()).expect(""),
		GradientType::Radial
	);
}
