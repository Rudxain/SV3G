#![warn(
	unused,
	future_incompatible,
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
#![allow(clippy::cargo_common_metadata)]
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

use std::{process::ExitCode, result, str::FromStr};

#[allow(clippy::wildcard_imports)]
use sv3g::*;

fn css_color_from_str(s: &str) -> CSSColor {
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

impl core::str::FromStr for SubCmds {
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

#[allow(clippy::too_many_lines)] // lmao
fn main() -> ExitCode {
	use std::env::args;
	const NAME: &str = "sv3g";

	let argv: Vec<String> = args().skip(1).collect();
	if argv.is_empty() {
		eprintln!("No arguments provided. Run `{NAME} help` for more info");
		return ExitCode::SUCCESS;
	}

	let subcmd = SubCmds::from_str(argv[0].as_str());
	if subcmd == Err(()) {
		eprintln!(
			"Unrecognized sub-command:\n{}\nRun `{NAME} help` to get list of valid ones",
			argv[0]
		);
		return ExitCode::FAILURE;
	};

	// is there a better way?
	let mut argv = argv;
	argv.remove(0);
	argv.shrink_to_fit();
	let argv = argv;

	// this feels redundant
	let subcmd = subcmd.unwrap();

	match subcmd {
		SubCmds::Help => {
			println!(
				"\
				usage: {NAME} <subcommand> [colors...]\n\
				help | HELP | man | /? | ❔ | ❓ | ℹ️ | ℹ : print this text\n\
				wb | WB: grayscale\n\
				rainbow | 🌈: RYGCBM\n\
				sky : like a skybox\n\
				mint | Mint : Linux Mint\n\
				fire | 🔥 : is it a candle?\n\
				custom : to specify arbitrary colors\
			"
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
			let colors: Vec<_> =
				argv.into_iter().map(CSSColor::new).collect();

			for r in &colors {
				match r {
					Ok(_) => continue,
					Err(e) => {
						eprint!("{e}");
						return ExitCode::FAILURE;
					}
				}
			}
			println!(
				"{}",
				generate(
					&GradientType::Linear,
					colors.into_iter().map(result::Result::unwrap).collect()
				)
			);
		}
	}

	ExitCode::SUCCESS
}
