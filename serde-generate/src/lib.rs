// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This crate provides experimental code generation in several languages for the formats
//! extracted by [`serde_reflection`](https://crates.io/crates/serde_reflection).
//!
//! ## Supported Languages
//!
//! * Python 3
//! * C++ 17
//! * Rust 2018
//!
//! ## Supported Encodings
//!
//! The code generated by this crate is meant to be used together with a runtime that
//! defines a particular [Serde encoding format](https://serde.rs/#data-formats).
//!
//! We currently only support binary formats similar to [Bincode](https://docs.rs/bincode/1.2.1/bincode/).
//!
//! ## Binary Tool
//!
//! Together with the library `serde_generate`, we provide a simple binary tool to process Serde formats
//! saved on disk.
//!
//! Assuming that a `serde_reflection::Registry` object has been serialized in a YAML file `test.yaml`,
//! the following command will generate Python class definitions and write them into `test.py`.
//!
//! ```bash
//! cargo run -p serde-generate -- --language python3 test.yaml > test.py
//! ```
//!
//! See the help message of the tool with `--help` for more options.
//!
//! ## Bincode Runtimes
//!
//! For testing purposes, we use the Bincode encoding format provided by the
//! [`bincode`](https://docs.rs/bincode/1.2.1/bincode/) crate in Rust and
//! provide experimental Bincode runtimes in Python and C++.
//!
//! In the following example, we transfer a `Test` value from Rust to Python using bincode.
//! ```
//! use serde::{Deserialize, Serialize};
//! use serde_reflection::{Registry, Samples, Tracer, TracerConfig};
//! use std::io::Write;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Test {
//!     a: Vec<u64>,
//!     b: (u32, u32),
//! }
//!
//! # fn main() -> Result<(), std::io::Error> {
//! // Obtain the Serde format of `Test`.
//! let mut tracer = Tracer::new(TracerConfig::default());
//! tracer.trace_type::<Test>(&Samples::new()).unwrap();
//! let registry = tracer.registry().unwrap();
//!
//! // Create Python class definitions.
//! let mut source = Vec::new();
//! serde_generate::python3::output(&mut source, &registry)?;
//! assert_eq!(
//! #  "\n".to_string() + &
//!     String::from_utf8_lossy(&source),
//!     r#"
//! from dataclasses import dataclass
//! import typing
//! import serde_types as st
//!
//! @dataclass
//! class Test:
//!     a: typing.Sequence[st.uint64]
//!     b: typing.Tuple[st.uint32, st.uint32]
//!
//! "#.to_string());
//!
//! // Append some test code to demonstrate Bincode deserialization
//! // using the runtime in `serde_generate/runtime/python/bincode`.
//! writeln!(
//!     source,
//!     r#"
//! import bincode
//!
//! value, _ = bincode.deserialize(bytes.fromhex("{}"), Test)
//! assert value == Test(a=[4, 6], b=(3, 5))
//! "#,
//!     hex::encode(&bincode::serialize(&Test { a: vec![4, 6], b: (3, 5) }).unwrap()),
//! )?;
//!
//! // Execute the Python code.
//! let mut child = std::process::Command::new("python3")
//!     .arg("-")
//!     .env("PYTHONPATH", std::env::var("PYTHONPATH").unwrap_or_default() + ":runtime/python")
//!     .stdin(std::process::Stdio::piped())
//!     .spawn()?;
//! child.stdin.as_mut().unwrap().write_all(&source)?;
//! let output = child.wait_with_output()?;
//! assert!(output.status.success());
//! # Ok(())
//! # }
//! ```

/// Dependency analysis and topological sort for Serde formats.
pub mod analyzer;
/// Support for code-generation in C++
pub mod cpp;
/// Support for code-generation in Python 3
pub mod python3;
/// Support for code-generation in Rust
pub mod rust;

#[doc(hidden)]
/// Utility functions to help testing code generators.
pub mod test_utils;

/// How to copy generated source code and available runtimes for a given language.
pub trait SourceInstaller {
    type Error;

    /// Create a module exposing the container types contained in the registry.
    fn install_module(
        &self,
        name: &str,
        registry: &serde_reflection::Registry,
    ) -> std::result::Result<(), Self::Error>;

    /// Install the serde runtime.
    fn install_serde_runtime(&self) -> std::result::Result<(), Self::Error>;

    /// Install the bincode runtime.
    fn install_bincode_runtime(&self) -> std::result::Result<(), Self::Error>;

    /// Install the Libra Canonical Serialization (LCS) runtime.
    fn install_lcs_runtime(&self) -> std::result::Result<(), Self::Error>;
}
