#[cfg(test)]
mod lib_test;

mod shell;

use clap::{Parser, arg};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use std::ffi::OsString;
use anyhow::Result;

#[allow(dead_code)]
pub mod built {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser, Debug)]
#[command(
    name = "oxipy",
    about = "An interactive Python environment with shell flavor.",
    version = built::GIT_VERSION,
    no_binary_name = true,
)]
pub struct Cli {
    /// program passed in as string
    #[arg(short = 'c')]
    pub command: Option<String>,

    // /// The RC files to load, these may be either xonsh files or directories containing xonsh files
    // #[arg(long)]
    // pub rc: Option<Vec<PathBuf>>,

    // /// Do not load any RC files. Argument --rc will be ignored if --no-rc is set
    // #[arg(long)]
    // pub no_rc: bool,

    /// Do not inherit program specific environment variables from parent process
    #[arg(long)]
    pub no_env: bool,

    /// Define an environment variable, in the form of -DNAME=VAL. May be used many times
    #[arg(short = 'D')]
    pub defines: Option<Vec<String>>,

    #[command(flatten)]
    verbose: Verbosity<WarnLevel>,

    // /// If present, execute the script in script-file and exit
    // #[arg()]
    // script_file: Option<PathBuf>,

    // /// Additional arguments to the script specified by script-file
    // #[arg()]
    // args: Vec<String>,
}

impl Cli {
    pub fn main<I, T>(args: I) -> Result<()>
    where
        I: IntoIterator<Item = T> + std::fmt::Debug,
        T: Into<OsString> + Clone,
    {
        let cli = Cli::parse_from(args);
        env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

        if let Some(command) = cli.command {
            log::info!("Running command: {}", command);
            return Ok(());
        }

        log::info!("Starting interactive shell");
        let mut shell = shell::Shell::new()?;
        // let mut shell = rustyline::DefaultEditor::new()?;
        shell.run()?;
        Ok(())
    }
}
