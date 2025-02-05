test:
	cargo clippy
	cargo nextest run --workspace --all-features
	uv run pytest
.PHONY: test

bench:
	pytest --codspeed tests/bench.py
.PHONY: bench
