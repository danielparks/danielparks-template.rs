#!/bin/zsh

set -e

export TEMPLATE_DIR=${0:A:h}
export TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

cargo generate --init --path "$TEMPLATE_DIR" "$@"

zsh

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
