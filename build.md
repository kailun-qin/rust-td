# Build Rust Td

## Tools

1. Install [RUST](https://www.rust-lang.org/)

please use nightly-2020-11-09.

1.1. Intall xbuild

```
cargo install cargo-xbuild
```

Please reinstall cargo-xbuild, after you update the rust toolchain.

2. Install [NASM](https://www.nasm.us/)

Please make sure nasm can be found in PATH.

3. Install LLVM

Please make sure clang can be found in PATH.

Set env:

```
set CC=clang
set AR=llvm-ar
```

## Build TdShim
```
cargo xbuild -p rust-tdshim --target x86_64-unknown-uefi --release
```

## Build PE format payload
```
pushd rust-td-payload
cargo xbuild --target x86_64-unknown-uefi --release
popd
cargo run -p rust-td-tool -- target/x86_64-unknown-uefi/release/ResetVector.bin target/x86_64-unknown-uefi/release/rust-tdshim.efi target/x86_64-unknown-uefi/release/rust-td-payload.efi target/x86_64-unknown-uefi/release/final.bin
```

## Build Elf format payload
```
pushd rust-td-payload
cargo xbuild --target target.json --release
popd
cargo run -p rust-td-tool -- target/x86_64-unknown-uefi/release/ResetVector.bin target/x86_64-unknown-uefi/release/rust-tdshim.efi target/target//release/rust-td-payload target/x86_64-unknown-uefi/release/final.bin
```
