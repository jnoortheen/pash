import pytest


@pytest.mark.benchmark
def test_script():
    from xonsh.main import premain

    premain(["-c", "echo 1"])


@pytest.mark.benchmark
def test_interactive_rl():
    import pexpect

    proc = pexpect.spawn('xonsh --interactive --no-rc --shell=rl')
    proc.expect(["@"])
    proc.sendline("echo 1")
    proc.expect("1")
    proc.terminate()


@pytest.mark.benchmark
def test_interactive_ptk():
    import pexpect

    proc = pexpect.spawn("xonsh --interactive --no-rc --shell=ptk")
    proc.expect(["@"])
    proc.sendline("echo 1")
    proc.expect("1")
    proc.terminate()
