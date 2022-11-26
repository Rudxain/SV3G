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

pub fn generate(t: GradientType, colors: Vec<&str>) -> &str {
	let t = t.to_string();

	return &colors.into_iter().map(|c| *c).collect::<Vec<&str>>().join("");
}
