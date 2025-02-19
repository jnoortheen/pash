# environment variables of child processes

- can be defined like `foo=bar` before a command or with `export foo=bar` command as in posix shells
- in Python context, they can set like `ox.env['foo'] = 'bar'` or `os.environ['foo'] = 'bar'`
- using them would be like `ox.env['foo']` in python context. The child process have their own way of accessing them.

# difference from xonsh

- simplified syntax for subprocesses
    - no `$[...]`/`![]`/`!()` syntaxes
    - `$()` -> captured output returned as a subclass of `list[str]` with special methods
    <!-- - `!(...)` -> captured output and error returned as an object that can be used in boolean context -->
    - `!cmd -args` -> IPython/psql like syntax to run the command without capturing output/error. Returns an object that can be used to get the process info (e.g. exit code, ...).
    - For capturing the error, forward it to stdout stream.
    - No `@(...)` or `@$(...)` syntaxes. Instead use `{}` to execute Python code.
    - No `g``` support. By default the arguments are expected to be globs.