use super::*;
use clap::Parser;
use insta::assert_debug_snapshot;

#[test]
fn test_cli_parser() {
    let args: Vec<&str> = vec![];
    assert_debug_snapshot!(Cli::parse_from(args), @r"
    Cli {
        command: None,
        rc: None,
        no_rc: false,
        no_env: false,
        defines: None,
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: None,
        args: [],
    }
    ");
}

#[test]
fn test_cli_with_command() {
    let args = vec!["-c", "print('Hello, world!')"];
    assert_debug_snapshot!(Cli::parse_from(args), @r#"
    Cli {
        command: Some(
            "print('Hello, world!')",
        ),
        rc: None,
        no_rc: false,
        no_env: false,
        defines: None,
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: None,
        args: [],
    }
    "#);
}

#[test]
fn test_cli_with_rc_files() {
    let args = vec!["--rc", "file1.xsh", "--rc", "file2.xsh"];
    let cli = Cli::parse_from(args);
    assert_debug_snapshot!(cli, @r#"
    Cli {
        command: None,
        rc: Some(
            [
                "file1.xsh",
                "file2.xsh",
            ],
        ),
        no_rc: false,
        no_env: false,
        defines: None,
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: None,
        args: [],
    }
    "#);
}

#[test]
fn test_cli_no_rc() {
    let args = vec!["--no-rc"];
    let cli = Cli::parse_from(args);
    assert_debug_snapshot!(cli, @r"
    Cli {
        command: None,
        rc: None,
        no_rc: true,
        no_env: false,
        defines: None,
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: None,
        args: [],
    }
    ");
}

#[test]
fn test_cli_no_env() {
    let args = vec!["--no-env"];
    let cli = Cli::parse_from(args);
    assert_debug_snapshot!(cli, @r"
    Cli {
        command: None,
        rc: None,
        no_rc: false,
        no_env: true,
        defines: None,
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: None,
        args: [],
    }
    ");
}

#[test]
fn test_cli_with_defines() {
    let args = vec!["-DNAME=VAL", "-DOTHER=VAL2"];
    let cli = Cli::parse_from(args);
    assert_debug_snapshot!(cli, @r#"
    Cli {
        command: None,
        rc: None,
        no_rc: false,
        no_env: false,
        defines: Some(
            [
                "NAME=VAL",
                "OTHER=VAL2",
            ],
        ),
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: None,
        args: [],
    }
    "#);
}

#[test]
fn test_cli_with_script_file() {
    let args = vec!["script.py"];
    let cli = Cli::parse_from(args);
    assert_debug_snapshot!(cli, @r#"
    Cli {
        command: None,
        rc: None,
        no_rc: false,
        no_env: false,
        defines: None,
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: Some(
            "script.py",
        ),
        args: [],
    }
    "#);
}

#[test]
fn test_cli_with_script_args() {
    let args = vec!["script.py", "arg1", "arg2"];
    let cli = Cli::parse_from(args);
    assert_debug_snapshot!(cli, @r#"
    Cli {
        command: None,
        rc: None,
        no_rc: false,
        no_env: false,
        defines: None,
        verbose: Verbosity {
            verbose: 0,
            quiet: 0,
            phantom: PhantomData<clap_verbosity_flag::WarnLevel>,
        },
        script_file: Some(
            "script.py",
        ),
        args: [
            "arg1",
            "arg2",
        ],
    }
    "#);
}
