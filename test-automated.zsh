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
		cargo generate --init --path "$TEMPLATE_DIR" --name foo --$kind --silent
		cargo fmt --check
		cargo +nightly clippy --all-features --all-targets --quiet -- -D warnings
		cargo test --all-features --quiet
	)
done
