.PHONY: all fmt lint test setup unsetup

all:
	cargo build

fmt:
	cargo fmt

lint:
	cargo clippy > clippy_report.txt 2>&1

test:
	test -z "$$(cargo fmt --check)"
	cargo test  # TODO coverity?

setup:
	ln -s ../../script/pre-push.sh .git/hooks/pre-push

unsetup:
	rm .git/hooks/pre-push
