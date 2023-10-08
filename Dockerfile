# 使用 Ubuntu 作為基底來編譯程式
FROM ubuntu:latest as builder

# 設置非交互式環境變數以避免安裝時的提示
ENV DEBIAN_FRONTEND=noninteractive

# 安裝 Rust 和其他必要的依賴項
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 

# 複製程式碼到容器中
WORKDIR /usr/src/qrcode-actix
COPY . .

# 使用 --release 建構程式
RUN /root/.cargo/bin/cargo build --release

# 使用一個輕量級的 image 來執行程式
FROM ubuntu:latest

# 設置非交互式環境變數以避免安裝時的提示
ENV DEBIAN_FRONTEND=noninteractive

# 安裝必要的共享庫
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 複製二進位檔到容器中
COPY --from=builder /usr/src/qrcode-actix/target/release/qrcode-actix /usr/local/bin/qrcode-actix

# 指定容器執行程式
CMD ["qrcode-actix"]
