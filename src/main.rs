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
#[derive(Debug, PartialEq)]
enum SubCmds {
	Help,
	/// white and black
	Wb([CSSColor; 2]),
	/// 🌈
	Rainbow([CSSColor; 6]),
	// skybox
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
			"wb" | "WB" => Ok(Self::Wb([
				css_color_from_str("#fff"),
				css_color_from_str("#000"),
			])),
			// I know, this is horrible
			"rainbow" | "🌈" => Ok(Self::Rainbow([
				css_color_from_str("#f00"),
				css_color_from_str("#ff0"),
				css_color_from_str("#0f0"),
				css_color_from_str("#0ff"),
				css_color_from_str("#00f"),
				css_color_from_str("#f0f"),
			])),
			"sky" => Ok(Self::Sky([
				css_color_from_str("#00e"),
				css_color_from_str("#07e"),
				css_color_from_str("#0ff"),
			])),
			"mint" | "Mint" => Ok(Self::Mint([
				css_color_from_str("#fff"),
				css_color_from_str("#0e1"),
			])),
			"fire" | "🔥" => Ok(Self::Fire([
				css_color_from_str("#000"),
				css_color_from_str("#700"),
				css_color_from_str("#f70"),
				css_color_from_str("#ff0"),
				css_color_from_str("#fff"),
			])),
			"custom" => Ok(Self::Custom),
			_ => Err(()),
		}
	}
}

fn print_known(c: &[CSSColor]) {
	println!("{}", generate(&GradientType::Linear, c.to_vec()));
}

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
					wb | WB: grayscale\n\
					rainbow | 🌈: RYGCBM\n\
					sky : like a skybox\n\
					mint | Mint : Linux Mint\n\
					fire | 🔥 : is it a candle?\n\
					custom : to specify arbitrary colors\
				",
					NAME
				);
			}
			SubCmds::Wb(c) | SubCmds::Mint(c) => {
				print_known(&c);
			}
			SubCmds::Rainbow(c) => {
				print_known(&c);
			}
			SubCmds::Sky(c) => {
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
