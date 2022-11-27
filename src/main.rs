#![warn(
	unused,
	future_incompatible,
	clippy::exit,
	clippy::unwrap_used,
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

use sv3g::*;

#[derive(Debug, PartialEq)]
enum SubCmds {
	Help,
	Wb,
	Rainbow,
	Rgb,
	Sky,
	Mint,
	Fire,
	Custom,
}

impl core::str::FromStr for SubCmds {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input.to_ascii_lowercase().as_str() {
			"help" | "man" | "/?" | "❔" | "❓" | "ℹ️" | "ℹ" => Ok(Self::Help),
			"wb" => Ok(Self::Wb),
			"rainbow" | "🌈" => Ok(Self::Rainbow),
			"rgb" => Ok(Self::Rgb),
			"sky" => Ok(Self::Sky),
			"mint" => Ok(Self::Mint),
			"fire" | "🔥" => Ok(Self::Fire),
			"custom" => Ok(Self::Custom),
			_ => Err(()),
		}
	}
}

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
			"Unrecognized sub-command:\n${}\nRun `{} help` to get list of valid ones",
			argv[0], NAME
		);
		return ExitCode::FAILURE;
	};

	#[allow(clippy::unwrap_used)]
	let subcmd = subcmd.unwrap();

	match subcmd {
		SubCmds::Help => {
			println!("\
				usage: {NAME} <subcommand> [colors...]\n\
				help | man | /? | ❔ | ❓ | ℹ️ | ℹ : print this text\n\
				wb : grayscale\n\
				rainbow | 🌈: RYGCBM\n\
				rgb : Red, Green, Blue\n\
				sky : like a skybox\n\
				mint : Linux Mint\n\
				fire | 🔥 : is it a candle?\n\
				custom : to specify arbitrary colors\
			")
		},
		_ => {}
	}

	ExitCode::SUCCESS
}
