#!/bin/bash
# Run: yes | ./scripts/upload_release.sh
#
# Don't forget to make a new release first with
# cargo release <major|minor|patch> [--execute] [-p unctool]
#

# Exit on error
set -e

latest_tag=$(git tag | grep unctool-cli | tail -n 1)
rm -rf gh_release && mkdir gh_release
cd gh_release

echo "Cloning repository with tag $latest_tag"
git clone --depth 1 --branch $latest_tag \
	https://github.com/poul1x/unctool.git

cd unctool
echo "Building for x86_64..."
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/unctool-cli unctool-x86_64
strip --strip-all ./unctool-x86_64

echo "Building for i686..."
rustup target add i686-unknown-linux-musl
cargo build --release --target i686-unknown-linux-musl
cp ./target/i686-unknown-linux-musl/release/unctool-cli unctool-i686
strip --strip-all ./unctool-i686

echo "Uploading files..."
gh release create $latest_tag
gh release upload $latest_tag ./unctool-x86_64
gh release upload $latest_tag ./unctool-i686
echo "Done"