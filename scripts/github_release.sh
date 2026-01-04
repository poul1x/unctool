#!/bin/bash
# Run: yes | ./scripts/upload_release.sh
#
# Don't forget to make a new release first with
# cargo release <major|minor|patch> [--execute] [-p unctool]
#

# Exit on error
set -e

rm -rf gh_release
mkdir gh_release
cd gh_release

echo "Cloning repository"
git clone --depth 1 https://github.com/poul1x/unctool.git
cd unctool

echo "Building CLI for Linux 64bit"
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl -p unctool-cli
cp ./target/x86_64-unknown-linux-musl/release/unctool-cli unctool-cli-x64

echo "Building CLI for Linux 32bit"
rustup target add i686-unknown-linux-musl
cargo build --release --target i686-unknown-linux-musl -p unctool-cli
cp ./target/i686-unknown-linux-musl/release/unctool-cli unctool-cli-x32

echo "Building GUI for Linux 64bit"
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu -p unctool-gui
cp ./target/x86_64-unknown-linux-gnu/release/unctool-gui unctool-gui-x64

echo "Building CLI for Linux 32bit"
rustup target add i686-unknown-linux-gnu
cargo build --release --target i686-unknown-linux-gnu -p unctool-gui
cp ./target/i686-unknown-linux-gnu/release/unctool-gui unctool-gui-x32

echo "Uploading files..."
tag="$(git log --oneline -1 --format="%h")-$(date -u +%Y%m%d-%H%M)"
gh release create $tag
gh release upload $tag ./unctool-cli-x64
gh release upload $tag ./unctool-cli-x32
gh release upload $tag ./unctool-gui-x64
gh release upload $tag ./unctool-gui-x32

cd ..
rm -rf gh_release
echo "Done"