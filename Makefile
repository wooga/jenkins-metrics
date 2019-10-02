DESTDIR =
PREFIX  = /usr/local

all: target/release/jenkins_metrics
build: target/release/jenkins_metrics

target/release/jenkins_metrics:
	cargo build --release --all

install: install-jenkins_metrics

install-jenkins_metrics: target/release/jenkins_metrics
	install -m755 -- target/release/jenkins_metrics "$(DESTDIR)$(PREFIX)/bin/"

test: target/release/jenkins_metrics
	cargo test --release $(CARGO_OPTS)

check: test

uninstall:
	-rm -f -- "$(DESTDIR)$(PREFIX)/bin/jenkins_metrics"

clean:
	cargo clean

help:
	@echo 'Available make targets:'
	@echo '  all         - build jenkins_metrics (default)'
	@echo '  build       - build jenkins_metrics'
	@echo '  clean       - run `cargo clean`'
	@echo '  install     - build and install jenkins_metrics'
	@echo '  test        - run `cargo test`'
	@echo '  uninstall   - uninstall uvm'
	@echo '  help        - print this help'
	@echo
	@echo
	@echo 'Variables:'
	@echo '  DESTDIR  - A path that'\''s prepended to installation paths (default: "")'
	@echo '  PREFIX   - The installation prefix for everything except zsh completions (default: /usr/local)'
	@echo '  FEATURES - The cargo feature flags to use. Set to an empty string to disable git support'

.PHONY: all build target/release/uvm install-uvm \
	clean uninstall help
