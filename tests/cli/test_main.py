import subprocess
from oxipy import __main__
import sys


def test_main(capfd):
    __main__.main(["-c", "echo hello", "-vv"])

    captured = capfd.readouterr()
    # with capfd.disabled():
    #     print(captured.err)
    assert "Running command" in captured.err


def test_oxcli_help():
    # Spawn the oxcli command with --help
    child = subprocess.run(
        [sys.executable, "-m", "oxipy", "--help"], capture_output=True, check=True
    )
    output = child.stdout.decode("utf-8")
    assert "Usage" in output
    assert "--help" in output
