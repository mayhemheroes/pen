#!/bin/sh

set -e

version=$($(dirname $0)/version.sh)
target=$(rustc -vV | grep host: | sed 's/host: //')
tarball=pen-$version-$target.tar.xz

cd $(dirname $0)/..

cargo build --release
strip target/release/pen

tar caf $tarball \
  README.md LICENSE.md LICENSE-MIT LICENSE-APACHE \
  doc lib rust-toolchain.toml target/release/pen

echo $tarball
