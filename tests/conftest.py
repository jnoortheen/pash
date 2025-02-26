import os

if not os.environ.get("GITHUB_ACTIONS"):
    import maturin_import_hook
    from maturin_import_hook.settings import MaturinSettings

    maturin_import_hook.install(
        settings=MaturinSettings(
            release=False,
            # uv=True,
        )
    )
import ast
import logging
from unittest.mock import MagicMock

import pytest

log = logging.getLogger(__name__)


@pytest.fixture
def parse_string():
    from oxipy import Parser

    def factory(text: str):
        return Parser(text).parse()

    return factory


@pytest.fixture
def parse_file():
    from oxipy import Parser

    def factory(path: str):
        return Parser.parse_file(path)

    return factory


@pytest.fixture
def unparse(unparse_diff, parse_string):
    def factory(text: str):
        left_tree = parse_string(text)
        return ast.unparse(left_tree)
        # from test_py_syntax import dump_diff
        # assert not dump_diff(parsed=left_tree, expected=right_tree), f"Generated AST didn't match. Source: {text}"

    return factory


@pytest.fixture
def unparse_diff(parse_string):
    def factory(text: str, right: str | None = None):
        left_tree = parse_string(text)
        left = ast.unparse(left_tree)
        right = right or left
        right_tree = ast.parse(right)
        assert left == ast.unparse(right_tree), f"unparse didn't match. Source: {text}"
        # from test_py_syntax import dump_diff
        # assert not dump_diff(parsed=left_tree, expected=right_tree), f"Generated AST didn't match. Source: {text}"

    return factory


@pytest.fixture
def xsh():
    obj = MagicMock()

    class Cmd:
        def __call__(self, *args, **kwargs):
            self.args = list(args)
            self.kwargs = kwargs
            self.result = None
            self.calls = []
            return self

        def _call(self, mode: str):
            self.result = self.args
            if self.kwargs:
                self.result.append(self.kwargs)
            self.calls.append(mode)
            return self.result

        def out(self):
            return self._call("out")

        def run(self):
            return self._call("run")

        def hide(self):
            return self._call("hide")

        def obj(self):
            return self._call("obj")

        def inject(self):
            return self._call("inject")

        def pipe(self, *args):
            self.args = [self.args, args]
            return self

    def list_of_strs_or_callables(x):
        """
        A simplified version of the xonsh function.
        """
        if isinstance(x, str | bytes):
            return [x]
        if callable(x):
            return [x(obj.cmd.args)]
        return list(x)

    # using instance to store the result
    obj.cmd = Cmd()
    obj.list_of_strs_or_callables = MagicMock(wraps=list_of_strs_or_callables)
    obj.call_macro = MagicMock()
    obj.enter_macro = MagicMock()
    return obj


@pytest.fixture
def exec_code(xsh, parse_string):
    """compatibility fixture"""

    def factory(
        inp: str,
        xenv: dict | None = None,
        mode="exec",
        **locs,
    ):
        xsh.obs = parse_string(inp)
        bytecode = compile(xsh.obs, "<test-xonsh-ast>", mode)
        xsh.env = xenv or {}
        globs = {"ox": xsh}
        locs.update(globs)
        try:
            exec(bytecode, globs, locs)
        except Exception:
            print(f"error execing {ast.unparse(xsh.obs)=}")
            raise
        return xsh

    return factory


@pytest.fixture
def cmd(exec_code):
    def factory(
        inp: str,
        xenv: dict | None = None,
        mode="exec",
        **locs,
    ):
        xsh = exec_code(inp, xenv, mode, **locs)
        return xsh.cmd.result

    return factory
