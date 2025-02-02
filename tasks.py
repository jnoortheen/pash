#!/usr/bin/env xonsh
import itertools
import subprocess
import sys

import xonsh.cli_utils as xcli
from xonsh.built_ins import XSH
from xonsh.tools import print_color

XSH.env["RAISE_SUBPROC_ERROR"] = True
XSH.env["COVERAGE_CORE"] = "sysmon"

def colored_tracer(cmds, **_):
    cmd = " ".join(itertools.chain.from_iterable(cmds))
    print_color(f"{{GREEN}}$ {{BLUE}}{cmd}{{RESET}}", file=sys.stderr)


def _replace_args(args: list[str], num: int) -> list[str]:
    return [
         (arg % num) if "%d" in arg else arg
         for arg in args
    ]


def test(
        report_cov: xcli.Arg('--report-coverage', '-c', action="store_true") = False,
        pytest_args: xcli.Arg(nargs='*')=(),
):
    """Run pytest.

    Parameters
    ----------
    report_cov
        Report coverage at the end of the test
    pytest_args
        arbitrary arguments that gets passed to pytest's invocation.
        Use %%d to parameterize and prevent overwrite

    Examples
    --------
    `xonsh tasks.py -- --junitxml=junit/test-results.%%d.xml`
    """

    if report_cov:
        XSH.subproc_uncaptured(["pytest", *_replace_args(pytest_args, 0), "--cov", "--cov-report=xml", "--cov-report=term"])
    else:
        # during CI run, some tests take longer to complete on windows
        XSH.subproc_uncaptured(["pytest", *_replace_args(pytest_args, 0), "--durations=5"])

def bench():
    """Run pytest-codspeed based benchmarks."""
    XSH.subproc_uncaptured(["pytest", "tests/bench.py", "--codspeed"])


if __name__ == '__main__':
    parser = xcli.make_parser("test commands")
    parser.add_command(test)
    parser.add_command(bench)

    try:
        xcli.dispatch(parser)
    except subprocess.CalledProcessError as ex:
        parser.exit(1, f"Failed with {ex}")
