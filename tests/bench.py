import pytest


@pytest.mark.benchmark
def test_script():
    from xonsh.main import premain

    premain(["-c", "echo 1"])


@pytest.mark.benchmark
def test_interactive_rl():
    from ptyprocess import PtyProcessUnicode as pty

    proc = pty.spawn(["xonsh", "--interactive", "--no-rc", "--shell=rl"])
    proc.readline()
    proc.write("echo 1\n")
    proc.terminate()


@pytest.mark.benchmark
def test_interactive_ptk():
    from ptyprocess import PtyProcessUnicode as pty

    proc = pty.spawn(["xonsh", "--interactive", "--no-rc", "--shell=ptk"])
    proc.readline()
    proc.write("echo 1\n")
    proc.terminate()
