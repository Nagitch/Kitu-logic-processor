# Rust公式イメージをベースにする
FROM rust:1.77

# 作業ディレクトリの設定
WORKDIR /usr/src

# ビルド時に必要なパッケージをインストール
RUN apt-get update && apt-get install -y git
RUN cargo install cargo-watch
RUN rustup update

# クロスコンパイルのためのツールをインストール

# Windows (x86_64)
# MinGW (GNU toolchain)
RUN apt install -y mingw-w64
RUN rustup target add x86_64-pc-windows-gnu
# MSVC (Microsoft toolchain)
RUN rustup target add x86_64-pc-windows-msvc

# Mac (Apple Silicon)
RUN rustup target add aarch64-apple-darwin

# Linux (x86_64)
RUN rustup target add x86_64-unknown-linux-gnu

# iOS (Apple)
# TODO: リンカーの設定:Xcodeのツールチェーン？
# Apple Silicon
RUN rustup target add aarch64-apple-ios
# Simulator
RUN rustup target add x86_64-apple-ios

# Android (Google)
# TODO: NDKをダウンロードしてパスを設定し、.cargo/config.tomlにリンカーのパスを明記
RUN rustup target add aarch64-linux-android    # 64-bit ARM
RUN rustup target add armv7-linux-androideabi  # 32-bit ARM
RUN rustup target add i686-linux-android       # 32-bit x86
RUN rustup target add x86_64-linux-android     # 64-bit x86

# WebAssembly (Wasm)
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack

# 以降のコマンドを作業ディレクトリで実行
COPY . .

# Rustプロジェクトのビルド
RUN cargo build
