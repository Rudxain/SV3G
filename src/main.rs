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

use std::{process::ExitCode, str::FromStr};

#[allow(clippy::wildcard_imports)]
use sv3g::*;

#[derive(Debug, PartialEq)]
enum SubCmds {
	Help,
	Wb([String; 2]),
	Rainbow([String; 6]),
	Rgb([String; 3]),
	Sky([String; 3]),
	Mint([String; 2]),
	Fire([String; 5]),
	Custom,
}

impl core::str::FromStr for SubCmds {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"help" | "HELP" | "man" | "/?" | "❔" | "❓" | "ℹ️" | "ℹ" => Ok(Self::Help),
			"wb" | "WB" => Ok(Self::Wb(["#fff".to_string(), "#000".to_string()])),
			// this is so redundant
			"rainbow" | "🌈" => Ok(Self::Rainbow([
				"#f00".to_string(),
				"#ff0".to_string(),
				"#0f0".to_string(),
				"#0ff".to_string(),
				"#00f".to_string(),
				"#f0f".to_string(),
			])),
			"rgb" | "RGB" => Ok(Self::Rgb([
				"#f00".to_string(),
				"#0f0".to_string(),
				"#00f".to_string(),
			])),
			"sky" => Ok(Self::Sky([
				"#00e".to_string(),
				"#07e".to_string(),
				"#0ff".to_string(),
			])),
			"mint" | "Mint" => Ok(Self::Mint(["#fff".to_string(), "#0e1".to_string()])),
			"fire" | "🔥" => Ok(Self::Fire([
				"#000".to_string(),
				"#700".to_string(),
				"#f70".to_string(),
				"#ff0".to_string(),
				"#fff".to_string(),
			])),
			"custom" => Ok(Self::Custom),
			_ => Err(()),
		}
	}
}

fn print_known(c: &[String]) {
	// to-do: validate const colors at compile-time
	println!("{}", generate(GradientType::Linear, c.to_vec()).unwrap());
}

#[allow(clippy::too_many_lines)] // lmao
fn main() -> ExitCode {
	use std::env::args;
	const NAME: &str = "sv3g";

	let argv: Vec<String> = args().skip(1).collect();
	if argv.is_empty() {
		eprintln!("No arguments provided. Run `{} help` for more info", NAME);
		return ExitCode::SUCCESS;
	}

	let subcmd = SubCmds::from_str(argv[0].as_str());
	if subcmd == Err(()) {
		eprintln!(
			"Unrecognized sub-command:\n{}\nRun `{} help` to get list of valid ones",
			argv[0], NAME
		);
		return ExitCode::FAILURE;
	};

	// is there a better way?
	let mut argv = argv;
	argv.remove(0);
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
				rgb | RGB: Red, Green, Blue\n\
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
		SubCmds::Rgb(c) | SubCmds::Sky(c) => {
			print_known(&c);
		}
		SubCmds::Fire(c) => {
			print_known(&c);
		}
		SubCmds::Custom => {
			match generate(GradientType::Linear, argv) {
				Ok(svg) => {
					println!("{}", svg);
				}
				Err(e) => {
					eprint!("{}", e);
					return ExitCode::FAILURE;
				}
			};
		}
	}

	ExitCode::SUCCESS
}
