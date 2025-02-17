use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline::{Completer, Helper, Hinter, Validator};
use anyhow::Result;

#[derive(Helper, Completer, Hinter, Validator)]
struct ShellHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Highlighter for ShellHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

pub(crate) struct Shell {
    editor: Editor<ShellHelper, DefaultHistory>,
    prompt: String,
    history_file: Option<String>,
}

impl Shell {
    pub(crate) fn new() -> Result<Self> {
        let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();
    let helper = ShellHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter::new(),
        colored_prompt: String::new(),
        validator: MatchingBracketValidator::new(),
    };
        let mut editor = Editor::with_config(config)?;
        editor.set_helper(Some(helper));

        // Set default history file in home directory
        let history_file = dirs::home_dir()
            .map(|mut path| {
                path.push(".oxsh_history");
                path.to_string_lossy().into_owned()
            });

        if let Some(ref history) = history_file {
            if editor.load_history(history).is_err() {
                log::warn!("No previous history.");
            }
        }

        Ok(Shell {
            editor,
            prompt: "oxsh> ".to_string(),
            history_file,
        })
    }

    pub(crate) fn run(&mut self) -> Result<()> {
        loop {
            match self.editor.readline(&self.prompt) {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Add to history
                    self.editor.add_history_entry(line)?;

                    // Handle built-in commands
                    match line {
                        "exit" | "quit" => break,
                        "history" => self.show_history(),
                        _ => self.execute_command(line),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    log::info!("^C");
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    log::error!("Error: {:?}", err);
                    break;
                }
            }
        }

        // Save history on exit
        if let Some(ref history) = self.history_file {
            let _ = self.editor.save_history(history);
        }
        Ok(())
    }

    fn show_history(&self) {
        let history = self.editor.history();
        for (i, entry) in history.iter().enumerate() {
            log::warn!("{:5} {}", i + 1, entry);
        }
    }

    fn execute_command(&self, command: &str) {
        let parts = command.split_whitespace();
        log::info!("Executing command: {:?} {}", parts.collect::<Vec<&str>>(), self.prompt);
        // if let Some(program) = parts.next() {
        //     let args: Vec<&str> = parts.collect();
        //     log::info!("Executing command: {:?}", command);
        //     // match Command::new(program)
        //     //     .args(&args)
        //     //     .stdout(Stdio::inherit())
        //     //     .stderr(Stdio::inherit())
        //     //     .spawn()
        //     // {
        //     //     Ok(mut child) => {
        //     //         match child.wait() {
        //     //             Ok(status) => {
        //     //                 if !status.success() {
        //     //                     if let Some(code) = status.code() {
        //     //                         eprintln!("Process exited with code: {}", code);
        //     //                     }
        //     //                 }
        //     //             }
        //     //             Err(e) => eprintln!("Error waiting for process: {}", e),
        //     //         }
        //     //     }
        //     //     Err(e) => eprintln!("Failed to execute command: {}", e),
        //     // }
        // }
    }
}
