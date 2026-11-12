# tessellix — developer shortcuts
#
# The project root is this directory (Cargo.toml lives here); from the git
# root one level up, use `make -C tessellix <target>`.
# See README.md (usage) and AGENTS.md (conventions) for details.

CARGO ?= cargo

# Export defaults (see `make list` for tiling ids).
TILING ?= truncated-hexagonal
W      ?= 1200
H      ?= 800
SCALE  ?= 1.0
ROT    ?= 0
MODE   ?= pattern
OUT    ?= tessellix.svg

.DEFAULT_GOAL := help
.PHONY: help build release check check-no-features clippy fmt fmt-check \
        test test-one gui tui list export clean

help: ## list available targets
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'

build: ## debug build of all crates
	$(CARGO) build

release: ## optimized build (opt-level 2)
	$(CARGO) build --release

check: ## repo bar: cargo check --workspace, must stay warning-free
	$(CARGO) check --workspace

check-no-features: ## build tess-tilings with every feature off (gating hygiene)
	$(CARGO) check -p tess-tilings --no-default-features

clippy: ## stricter lint bar: zero warnings across all targets
	$(CARGO) clippy --workspace --all-targets

fmt: ## format (advisory; compact data tables are hand-tuned)
	$(CARGO) fmt

fmt-check: ## show what fmt would change (no edits)
	$(CARGO) fmt --check

test: ## full suite (~230 tests, ~17 s) — must stay green
	$(CARGO) test --workspace

test-one: ## run tests matching a name: make test-one T=square_covers_region
	@test -n "$(T)" || { echo "usage: make test-one T=<test name>"; exit 1; }
	$(CARGO) test --workspace $(T)

gui: ## desktop GUI (egui window) — needs a display
	$(CARGO) run -p tess-gui

tui: ## terminal UI — needs a REAL terminal (raw mode); not for headless scripts
	$(CARGO) run -p tess-tui

list: ## print the tiling registry (id, name, description)
	$(CARGO) run -q -p tess-tui -- list

export: ## write an SVG: make export TILING=... OUT=... [W H SCALE ROT MODE]
	$(CARGO) run -q -p tess-tui -- export \
		--tiling $(TILING) --width $(W) --height $(H) \
		--scale $(SCALE) --rotation $(ROT) --mode $(MODE) --out $(OUT)

clean: ## remove build artifacts
	$(CARGO) clean