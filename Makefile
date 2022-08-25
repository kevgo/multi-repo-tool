
fix:  # auto-corrects issues
	dprint fmt
	cargo fmt
	cargo fix

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --path .

lint: tools/actionlint  # checks formatting
	dprint check
	cargo clippy --all-targets --all-features -- -W clippy::pedantic
	cargo fmt -- --check
	git diff --check
	tools/actionlint

test: unit lint  # runs all tests

unit:  # runs the unit tests
	cargo test

tools/actionlint:
	curl -s https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash | bash
	mkdir -p tools
	mv actionlint tools

update:  # updates the dependencies
	cargo upgrade

.DEFAULT_GOAL := help
.SILENT:
