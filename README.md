# UNC Tool

Seamlessly convert between Linux and Windows UNC paths. Convert local Linux path to Windows/Linux UNC and vice versa.

## Usage

Convert between Linux and Windows UNC:

```bash
unctool convert 'smb://mynas.local/some/path' -t windows
# \\mynas.local\some\path

unctool convert '\\mynas.local\some\path' -t linux
# smb://mynas.local/some/path
```

Convert to remote UNC:

```bash
unctool remote-path /mnt/mynas.local/some/path -t windows
# \\mynas.local\some\path

unctool remote-path /mnt/mynas.local/some/path -t linux
# smb://mynas.local/some/path
```

Convert from remote UNC:

```bash
unctool local-path '\\mynas.local\some\path'
# /mnt/mynas.local/some/path

unctool local-path 'smb://mynas.local/some/path'
# /mnt/mynas.local/some/path
```

## How it works

UNC Tool reads `/proc/mounts`, filters CIFS mounts, and performs local/remote path substitutions. Conversion between Windows and Linux UNC paths handles OS separator replacement.

## Installation

### Linux 64-bit:

```bash
curl -sL -o unctool https://github.com/poul1x/unctool/releases/latest/download/unctool-x86_64
chmod +x unctool
sudo mv unctool /usr/local/bin

# Test run
unctool --help
```

### Linux 32-bit:

```bash
curl -sL -o unctool https://github.com/poul1x/unctool/releases/latest/download/unctool-i686
chmod +x unctool
sudo mv unctool /usr/local/bin

# Test run
unctool --help
```

## Build from sources

### Linux 64-bit:

```bash
git clone https://github.com/poul1x/unctool.git
cd unctool

rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

cp ./target/x86_64-unknown-linux-musl/release/unctool unctool
strip --strip-all ./unctool

# Test run
./unctool --help
```

### Linux 32-bit:

```bash
git clone https://github.com/poul1x/unctool.git
cd unctool

rustup target add i686-unknown-linux-musl
cargo build --release --target i686-unknown-linux-musl

cp ./target/i686-unknown-linux-musl/release/unctool unctool
strip --strip-all ./unctool

# Test run
./unctool --help
```