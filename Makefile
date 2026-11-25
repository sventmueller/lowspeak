# Entry points only; the build logic lives in xtask/src.
.DEFAULT_GOAL := help
.PHONY: help build deploy universal lint check test-intel test-models test-online test-devices release

help: ## List the targets
	@awk 'BEGIN {FS = ":.*## "} /^[a-z-]+:.*## / {printf "  %-12s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build target/bundle/Lowspeak.app at the version in Cargo.toml, signed for this Mac
	cargo xtask bundle

deploy: ## Raise the build number in Cargo.toml, build, replace the installed copy and launch it, pack target/bundle/Lowspeak-<version>.dmg (not notarized), commit the version
	cargo xtask deploy

universal: ## Build target/universal/Lowspeak for Apple Silicon and Intel, unsigned, as the disk image carries it; needs rustup
	cargo xtask universal

lint: ## Formatting and clippy, warnings fail
	cargo fmt --all --check
	cargo clippy --locked --release --workspace --all-targets -- -D warnings

check: lint ## lint, then every test that needs no model, key, network or microphone, with Cargo.lock as committed; what CI runs
	cargo test --locked --release --workspace

test-intel: ## The tests of make check as Intel code, which Rosetta runs here; built as the disk image's program, needs rustup
	cargo xtask intel-tests

test-models: ## Transcription with the installed models, and the fallback from an invalid key to Parakeet Ultra
	cargo test --release --test transcription -- --ignored
	cargo test --release --test fallback -- --ignored

test-online: ## OpenAI and Groq with the stored keys, and the model downloads from Hugging Face (about 700 MB)
	cargo test --release --test online -- --ignored
	cargo test --release --test downloads -- --ignored

test-devices: ## The default microphone; the terminal needs microphone access
	cargo test --release --test recording -- --ignored

release: check ## After make check, with RELEASE_NOTES.md and release-notes/<language>.md written: raise the build number, dist/Lowspeak-<version>.dmg, notarized, commit the version; replaces the old one
	cargo xtask dmg
