# Rust公式イメージをベースにする
FROM rust:1.77

# 作業ディレクトリの設定
WORKDIR /usr/src

# ビルド時に必要なパッケージをインストール
RUN apt-get update && apt-get install -y git
RUN cargo install cargo-watch

# 以降のコマンドを作業ディレクトリで実行
COPY . .

# Rustプロジェクトのビルド
RUN cargo build

# コンテナ起動時のデフォルトコマンド
# CMD ["cargo", "run"]
