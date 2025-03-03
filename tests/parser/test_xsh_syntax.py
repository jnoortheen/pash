"""Tests the xonsh parser."""

import pytest
from pathlib import Path

data = Path(__file__).parent.parent.joinpath("data")


@pytest.mark.yaml_snaps(data / "exprs.yml", data / "stmts.yml")
def test_line_items(snap, unparse):
    snap.matches(unparse(snap.inp))


@pytest.mark.parametrize(
    "inp",
    [
        'x = "WAKKA"; ${x} = 65',
        'x = "."; $(ls @(None or x))',
        'x = "."; !(ls @(None or x))',
        '$(git commit -am "wakka jawaka" )',
        '$(git commit -am "flock jawaka milwaka" )',
        '$(git commit -am "wakka jawaka")',
        '$(git commit -am "flock jawaka")',
        '!git commit -am "wakka jawaka" ',
        '!git commit -am "flock jawaka milwaka" ',
        '!git commit -am "wakka jawaka" ',
        '!git commit -am "flock jawaka" ',
    ],
)
@pytest.mark.xfail
def test_statements(exec_code, inp):
    exec_code(inp, mode="exec")


@pytest.mark.parametrize(
    "inp, result",
    [
        ("$(ls)", ["ls"]),
        ("$(ls )", ["ls"]),
        ("$( ls )", ["ls"]),
        ("$( ls)", ["ls"]),
        ("$(ls .)", ["ls", "."]),
        ('$(ls ".")', ["ls", "."]),
        ("$(ls -l)", ["ls", "-l"]),
        ("$(ls $WAKKA)", ["ls", "wak"]),
        ('$(ls @(None or "."))', ["ls", ["."]]),
        (
            '$(echo hello | @(lambda a, s=None: "hey!") foo bar baz)',
            [["echo", "hello"], (["hey!"], "foo", "bar", "baz")],
        ),
        pytest.param(
            "$(echo @(i**2 for i in range(20) ) )",
            ["echo", [i**2 for i in range(20)]],
        ),
        ("$(echo @('a', 7))", ["echo", ["a", 7]]),
        (
            "$(@$(which echo) ls | @(lambda a, s='stdin': $(@(s.strip()) @(a[1]))) foo -la baz)",
            [[["stdin"], ["ls"]], ([[["stdin"], ["ls"]]], "foo", "-la", "baz")],
        ),
        ("$(ls $(ls))", ["ls", ["ls"]]),
        ("$(ls $(ls) -l)", ["ls", ["ls"], "-l"]),
    ],
)
def test_captured_procs(inp, result, exec_code):
    sh = exec_code(inp, xenv={"WAKKA": "wak"})
    assert sh.cmd.result == result

    last_call = sh.cmd.calls[-1]
    match inp[:2]:
        case "$[":
            assert last_call == "run"
        case "$(":
            assert last_call == "out"
        case "![":
            assert last_call == "hide"
        case "!(":
            assert last_call == "obj"


@pytest.mark.parametrize("p", ["", "p"])
@pytest.mark.parametrize("f", ["", "f"])
@pytest.mark.parametrize("glob_type", ["", "r", "g"])
@pytest.mark.xfail
def test_backtick(p, f, glob_type, exec_code):
    exec_code(f"print({p}{f}{glob_type}`.*`)")
