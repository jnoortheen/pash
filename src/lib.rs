#![allow(clippy::useless_conversion)]

mod annotate_src;
mod lexer;
mod location;
pub mod parser;
mod parser_test;

use oxipy_cli::{Cli, built};
use pyo3::prelude::*;
use pyo3_built::pyo3_built;

/// A Python module implemented in Rust.
#[pymodule(name = "_oxipy")]
mod oxipy {
    use super::*;

    #[pymodule_export]
    use parser::PyParser;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let py = m.py();
        m.add("__build__", pyo3_built!(py, built, "git", "build"))?;
        Ok(())
    }

    #[pyfunction] // This will be part of the module
    #[pyo3(signature = (*args))]
    fn cli_main(args: Vec<String>) -> anyhow::Result<()> {
        Cli::main(args)
    }
}
