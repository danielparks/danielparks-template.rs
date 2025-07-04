#!/bin/zsh

set -e

generate_args=()
case $1 in
	--bin)
		generate_args=(--bin --name foo-bin)
		shift
		;;
	--lib)
		generate_args=(--lib --name foo-lib)
		shift
		;;
	*)
		echo 'Expect --bin or --lib as first argument.' >&2
		exit 1
esac

export TEMPLATE_DIR=${0:A:h}
export TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

cargo generate --init --path "$TEMPLATE_DIR" $generate_args

zsh "$@"

cd
echo -n 'Keep temporary directory "'"$TEST_DIR"'" [yN]? '
read yn
case "$yn" in
	[yY]*) echo Keeping directory. ;;
	*)
		echo Removing directory.
		rm -rf -- "$TEST_DIR"
	;;
esac
