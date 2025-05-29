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

default_kinds=(lib bin)
for kind in ${*:-$default_kinds} ; do
	(
		mkdir foo-$kind
		cd foo-$kind
		set -x
		export RUSTFLAGS='-D warnings'
		cargo generate --init --path "$TEMPLATE_DIR" --name foo --$kind --silent
		cargo fmt --check
		cargo clippy --all-targets --all-features --quiet
		cargo deny --all-features check
		cargo test --all-features --quiet
		if [[ $kind == "bin" ]] ; then
			cargo run --quiet -- -vvv
		fi
	)
done
