build::
	cargo build

ci:: build lint-enforce
	echo ok

run::
	cargo run

publish:: build lint-enforce
	cargo publish

lint::
	cargo clippy --all
	cargo fmt --all

lint-enforce::
	cargo clippy --all -- -D warnings
	cargo fmt --all -- --check
