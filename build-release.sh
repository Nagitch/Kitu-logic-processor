# TODO: Unsupported targets

# Windows
cargo build --target x86_64-pc-windows-gnu --release
# cargo build --target x86_64-pc-windows-msvc --release

# Linux
cargo build --target x86_64-unknown-linux-gnu --release

# WASM
wasm-pack build --target web

# MacOS
# cargo build --target aarch64-apple-darwin --release

# iOS
# cargo build --target aarch64-apple-ios --release # iPhone
# cargo build --target x86_64-apple-ios --release # Simulator

# Android
# cargo build --target aarch64-linux-android --release
# cargo build --target armv7-linux-androideabi --release
# cargo build --target i686-linux-android --release
# cargo build --target x86_64-linux-android --release
