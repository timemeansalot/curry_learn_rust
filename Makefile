default: run_without_env

clean:
	cargo clean

run_without_env:
	cargo run

run_with_env:
	export IS_FALG=true && cargo run

.phony: run clean
