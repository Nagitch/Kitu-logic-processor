# Rust公式イメージをベースにする
FROM rust:1.62

# 作業ディレクトリの設定
WORKDIR /usr/src/kitu-logic-processor

# ビルド時に必要なパッケージをインストール
RUN apt-get update && apt-get install -y git

# 以降のコマンドを作業ディレクトリで実行
COPY . .

# Rustプロジェクトのビルド
RUN cargo build

# コンテナ起動時のデフォルトコマンド
# CMD ["cargo", "run"]
