"""Implements the xonsh parser."""

from xonsh.lib.lazyasd import lazyobject


@lazyobject
def Parser():
    from xonsh.parsers.rd_parser import Parser as p

    return p
