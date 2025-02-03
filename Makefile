test:
	cargo clippy
	cargo nextest run --workspace --all-features
	uv run pytest
.PHONY: test

bench:
	pytest --codspeed tests/bench.py
	# pytest tests/test_simple.py --memray
	python tests/bench_mem.py --empty
	python tests/bench_mem.py
.PHONY: bench
