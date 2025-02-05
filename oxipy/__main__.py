from ._oxipy import cli_main


def main(args: list[str] | None = None):
    import sys

    args = args or sys.argv[:]
    cli_main(*args)


if __name__ == "__main__":
    main()
