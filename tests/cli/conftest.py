import pytest
import sys


@pytest.fixture(scope="session")
def platform():
    return sys.platform


@pytest.fixture(scope="session")
def is_windows(platform):
    return platform == "win32"


@pytest.fixture
def process(is_windows):
    children = []

    def call(*args, **kwargs):
        if is_windows:
            import wexpect

            child = wexpect.spawn(*args, **kwargs)
        else:
            import pexpect

            child = pexpect.spawn(*args, **kwargs)
        children.append(child)

        return child

    yield call
    for child in children:
        child.terminate()
