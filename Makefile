.PHONY: build test fmt lint fix docs bench playground ci release check clean install

# Owner: PascalElixir / axolrs (GitHub org)
# File: Makefile - canonical project targets for the Axolotl workspace.

build:
	cargo build --workspace --exclude zed-axolotl

check:
	cargo check --workspace

test:
	cargo test --workspace --no-fail-fast

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace -- -D warnings

fix:
	cargo clippy --workspace --fix --allow-dirty --allow-staged

docs:
	cd axolsite && pnpm run build

playground:
	scripts/build-playground-wasm.sh

bench:
	cd benchmarks && for d in */; do if [ -d "$$d/rust" ]; then (cd "$$d/rust" && cargo run --release) || true; fi; done

ci: check lint test docs

release: ci
	cargo build --release --workspace --exclude zed-axolotl

clean:
	cargo clean
	rm -rf axolsite/.svelte-kit axolsite/dist axolsite/node_modules

install:
	cargo install --path crates/axolc
	cargo install --path crates/bucket
	cargo install --path crates/axol-analyzer
	cargo install --path crates/axol-hot-runner
