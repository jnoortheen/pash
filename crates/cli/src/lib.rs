use argh::FromArgs;

use shadow_rs::shadow;

shadow!(build);

/// Program description for your CLI tool
#[derive(FromArgs)]
struct Args {
    /// path to the configuration file
    #[argh(option)]
    config: Option<String>,

    /// verbose output mode
    #[argh(switch)]
    verbose: bool,

    /// an optional subcommand
    #[argh(subcommand)]
    command: Option<SubCommands>,
}
