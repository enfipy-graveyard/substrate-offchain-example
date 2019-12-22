//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;

pub use sc_cli::{error, IntoExit, VersionInfo};

fn main() -> Result<(), cli::error::Error> {
	let version = VersionInfo {
		name: "Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "example",
		author: "Some Team",
		description: "Some node",
		support_url: "support.example.com",
	};

	cli::run(std::env::args(), cli::Exit, version)
}
