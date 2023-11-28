# dev tooling and versions
ACTIONLINT_VERSION = 1.6.26
DPRINT_VERSION = 0.43.1
RUN_THAT_APP_VERSION = 0.5.0


cuke:  # runs the feature tests
	cargo build
	cargo test --test cucumber

cukethis: target/debug/mrt  # runs only end-to-end tests with a @this tag
	cargo build
	cargo test --test cucumber -- -t @this

fix: tools/run-that-app@${RUN_THAT_APP_VERSION}  # auto-corrects issues
	tools/rta dprint@${DPRINT_VERSION} fmt
	cargo fmt
	cargo fix

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary in the system
	cargo install --path .

lint: tools/actionlint  # checks formatting
	tools/rta dprint@${DPRINT_VERSION} check
	cargo clippy --all-targets --all-features -- --deny=warnings
	cargo fmt -- --check
	git diff --check
	tools/rta actionlint@${ACTIONLINT_VERSION}

test: unit lint cuke  # runs all tests

unit:  # runs the unit tests
	cargo test

update:  # updates the dependencies
	cargo upgrade

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/run-that-app@${RUN_THAT_APP_VERSION}:
	@echo "Installing run-that-app ${RUN_THAT_APP_VERSION} ..."
	@rm -f tools/run-that-app* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/run-that-app tools/run-that-app@${RUN_THAT_APP_VERSION}
	@ln -s run-that-app@${RUN_THAT_APP_VERSION} tools/rta

.DEFAULT_GOAL := help
.SILENT:
