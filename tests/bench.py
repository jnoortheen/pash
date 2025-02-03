import io

import pytest


@pytest.fixture
def ptk_input():
    from prompt_toolkit.application import create_app_session
    from prompt_toolkit.input import create_pipe_input

    out = io.StringIO()
    with (
        create_pipe_input() as pipe_input,
    ):
        from prompt_toolkit.output.plain_text import PlainTextOutput

        with create_app_session(input=pipe_input, output=PlainTextOutput(out)):
            yield pipe_input, out


@pytest.fixture
def call_main():
    def _call(args):
        from xonsh.main import main_xonsh, premain

        main_xonsh(premain(args))
        # return capsys.readouterr()

    return _call


@pytest.mark.benchmark
def test_script(call_main):
    _outerr = call_main(["-c", "echo 1"])
    # assert outerr.out == "1\n"


class TestInteractive:
    def interactive(self, shell: str):
        import pexpect

        proc = pexpect.spawn(f"xonsh --interactive --no-rc --shell={shell}")
        proc.expect(["@"])
        proc.sendline("echo 1")
        proc.expect("1")
        proc.terminate()

    @pytest.mark.benchmark
    def test_interactive_rl(self):
        self.interactive("rl")

    @pytest.mark.benchmark
    def test_interactive_ptk(self):
        self.interactive("ptk")


class TestMockedReading:
    """test in the same process"""

    @pytest.mark.benchmark
    def test_ptk(self, call_main, ptk_input):
        inp, _out = ptk_input
        inp.send_text("echo 1; quit\n")
        call_main(["-i", "--shell=ptk", "--no-rc"])
        # errout = capsys.readouterr()
        # assert out.read() == "1\n"
