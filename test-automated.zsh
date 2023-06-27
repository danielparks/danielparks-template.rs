#!/bin/zsh

set -e

export TEMPLATE_DIR=${0:A:h}
export TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

clean-up () {
	cd /
	rm -rf "$TEST_DIR"
}

trap clean-up EXIT

for kind in bin lib ; do
	(
		mkdir foo-$kind
		cd foo-$kind
		set -x
		export RUSTFLAGS='-D warnings'
		cargo generate --init --path "$TEMPLATE_DIR" --name foo --$kind --silent
		cargo fmt --check
		cargo lints clippy --all-targets --all-features --quiet
		cargo test --all-features --quiet
	)
done
