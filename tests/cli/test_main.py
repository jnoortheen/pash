import subprocess
from oxipy import __main__


def test_main(capfd):
    __main__.main(["-c", "echo hello", "-vv"])

    captured = capfd.readouterr()
    # with capfd.disabled():
    #     print(captured.err)
    assert "Running command" in captured.err


def test_oxcli_help():
    # Spawn the oxcli command with --help
    child = subprocess.run(["python", "-m", "oxipy", "--help"], capture_output=True)
    output = child.stdout.decode("utf-8")
    assert "Usage" in output
    assert "--help" in output
