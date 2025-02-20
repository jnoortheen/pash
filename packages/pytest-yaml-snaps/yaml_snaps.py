import os
import pytest
from pathlib import Path

from dataclasses import dataclass


@dataclass
class LineItem:
    path: Path
    exp: str
    key: str
    idx: int
    inp: str
    write: bool = False

    def matches(self, other: str) -> None:
        if self.write:
            self.write_yaml(other)
            assert False
        assert other == self.exp

    def __repr__(self):
        return repr(self.exp)

    def write_yaml(self, exp: str):
        from ruamel.yaml import YAML

        yaml = YAML()

        with self.path.open("r") as file:
            data = yaml.load(file)
        data[self.key][self.idx]["exp"] = exp
        yaml.dump(data, self.path.open("w"))


def get_cases(path: Path, write=False):
    from ruamel.yaml import YAML

    yaml = YAML()
    with path.open("r") as file:
        data = yaml.load(file)
    for case, lines in data.items():
        for idx, item in enumerate(lines):
            kwargs = dict()
            if case.startswith("_"):
                kwargs["marks"] = pytest.mark.xfail
            li = LineItem(
                path, item.get("exp", ""), case, idx, inp=item["inp"], write=write
            )
            yield pytest.param(li, id=f"{path.stem}-{case}-{idx}", **kwargs)


def get_files_from_markers(markers: list, write=False):
    for mark in markers:
        for path in mark.args:
            if not os.path.isfile(path):
                raise FileNotFoundError(f"File {path} does not exist")
            yield from get_cases(Path(path), write)


def pytest_generate_tests(metafunc):
    """Dynamically generate test cases for each line in each file."""
    if "snap" in metafunc.fixturenames:
        markers = [mark for mark in metafunc.definition.iter_markers(name="yaml_snaps")]
        if not markers:
            return
        write = metafunc.config.getoption("--update-snaps")
        test_cases = list(get_files_from_markers(markers, write))
        metafunc.parametrize("snap", test_cases)


def pytest_addoption(parser):
    parser.addoption(
        "--update-snaps",
        action="store_true",
        default=False,
        help="update the corresponding yaml snapshots",
    )


def pytest_configure(config):
    config.addinivalue_line(
        "markers",
        "yaml_snaps(*path): Parameterize the test with the line items in the yaml file(s) at path(s).",
    )
