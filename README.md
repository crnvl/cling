[![Check Build](https://github.com/angelsflyinhell/cling/actions/workflows/check.yml/badge.svg)](https://github.com/angelsflyinhell/cling/actions/workflows/check.yml)

# cling
Lightweight CLI client for ping instances. Also supports instances hosted elsewhere as long as they're not modified.

# How to use
### Option 1 (recommended)
0. Install [Rust](https://www.rust-lang.org/tools/install)
1. Clone this repository
2. Run `cargo build --release`
3. Copy the `config.json` file to the same directory as the binary
4. Run the binary at `target/release/cling` or `target/release/cling.exe` depending on your system
> I strongly recommend to build it yourself as prebuilt binaries might not be compatible with your system.

### Option 2
1. Go to [Releases](https://github.com/angelsflyinhell/cling/releases)
2. Download the latest release for your system
3. Copy the `config.json` file to the same directory as the binary
4. Run the binary
> This is the easier option, but specific binaries might not run on your system.