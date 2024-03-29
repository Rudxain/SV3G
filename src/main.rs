#![warn(
	unused,
	future_incompatible,
	clippy::unwrap_used,
	clippy::exit,
	clippy::cargo,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string
)]
// `uninlined_format_args` is part of `pedantic`, not `style`
// https://github.com/rust-lang/rust-clippy/issues/10082
#![allow(clippy::cargo_common_metadata, clippy::uninlined_format_args)]
#![forbid(
	unsafe_code,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::cast_precision_loss,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]
use core::str::FromStr;
use std::process::ExitCode;

#[allow(clippy::wildcard_imports)]
use sv3g::*;

fn css_color_from_str(s: &str) -> CSSColor {
	#[allow(clippy::unwrap_used)]
	CSSColor::new(s.to_string()).unwrap()
}

// I decided to use wrapper variants,
// because they are better for compile-time checks (AFAIK)
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
enum SubCmds {
	Help,
	Black([CSSColor; 1]),
	/// white and black
	WB([CSSColor; 2]),
	/// 🌈
	Rainbow([CSSColor; 6]),
	/// yellow cyan magenta
	YCM([CSSColor; 3]), // YMCA!!!
	/// skybox
	Sky([CSSColor; 3]),
	Mint([CSSColor; 2]),
	Fire([CSSColor; 5]),
	Custom,
}

impl FromStr for SubCmds {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"help" | "HELP" | "man" | "/?" | "❔" | "❓" | "ℹ️" | "ℹ" => Ok(Self::Help),
			"black" => Ok(Self::Black([css_color_from_str("#000")])),
			"wb" | "WB" => Ok(Self::WB(["#fff", "#000"].map(css_color_from_str))),
			// I know, this is horrible
			"rainbow" | "🌈" => Ok(Self::Rainbow(
				["#f00", "#ff0", "#0f0", "#0ff", "#00f", "#f0f"].map(css_color_from_str),
			)),
			"ycm" | "YCM" => Ok(Self::YCM(["#ff0", "#0ff", "#f0f"].map(css_color_from_str))),
			"sky" => Ok(Self::Sky(["#00e", "#07e", "#0ff"].map(css_color_from_str))),
			"mint" | "Mint" => Ok(Self::Mint(["#fff", "#0e1"].map(css_color_from_str))),
			"fire" | "🔥" => Ok(Self::Fire(
				["#000", "#700", "#f70", "#ff0", "#fff"].map(css_color_from_str),
			)),
			"custom" => Ok(Self::Custom),
			_ => Err(()),
		}
	}
}

fn print_known(c: &[CSSColor]) {
	println!("{}", generate(&GradientType::Linear, c.to_vec()));
}

#[allow(clippy::too_many_lines)]
fn main() -> ExitCode {
	use std::env::args;
	const NAME: &str = "sv3g";

	if args().count() < 2 {
		eprintln!("No arguments provided. Run `{} help` for more info", NAME);
		return ExitCode::SUCCESS;
	}

	let arg1: String = args().skip(1).take(1).collect();

	if let Ok(subcmd) = SubCmds::from_str(arg1.as_str()) {
		match subcmd {
			SubCmds::Help => {
				println!(
					"\
					usage: {} <subcommand> [colors...]\n\
					help | HELP | man | /? | ❔ | ❓ | ℹ️ | ℹ : print this text\n\
					black : pitch black\n\
					wb | WB : grayscale\n\
					rainbow | 🌈 : RYGCBM\n\
					ycm | YCM : yellow cyan magenta\n\
					sky : like a skybox\n\
					mint | Mint : Linux Mint\n\
					fire | 🔥 : is it a candle?\n\
					custom : to specify arbitrary colors\
				",
					NAME
				);
			}
			SubCmds::Black(c) => {
				print_known(&c);
			}
			SubCmds::WB(c) | SubCmds::Mint(c) => {
				print_known(&c);
			}
			SubCmds::Rainbow(c) => {
				print_known(&c);
			}
			SubCmds::Sky(c) | SubCmds::YCM(c) => {
				print_known(&c);
			}
			SubCmds::Fire(c) => {
				print_known(&c);
			}
			SubCmds::Custom => {
				let colors: Vec<_> = args().skip(2).into_iter().map(CSSColor::new).collect();

				for r in &colors {
					match r {
						Ok(_) => continue,
						Err(e) => {
							eprintln!("{}", e);
							return ExitCode::FAILURE;
						}
					}
				}
				println!(
					"{}",
					generate(
						&GradientType::Linear,
						colors
							.into_iter()
							.map(core::result::Result::unwrap)
							.collect()
					)
				);
			}
		}
		return ExitCode::SUCCESS;
	}
	eprintln!(
		"Unrecognized sub-command:\n{}\nRun `{} help` to get list of valid ones",
		arg1, NAME
	);
	ExitCode::FAILURE
}
