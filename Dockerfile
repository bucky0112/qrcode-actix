# 使用 Ubuntu 20.04 映像作為基底映像來編譯應用程式
FROM ubuntu:latest as builder

# 安裝 Rust 和其他必要的依賴項
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 

# 複製您的源程式碼到容器中
WORKDIR /usr/src/qrcode-actix
COPY . .

# 使用 --release 旗標建構您的應用程式
RUN /root/.cargo/bin/cargo build --release

# 使用一個輕量級的基底映像來執行您的應用程式
FROM ubuntu:latest

# 安裝必要的共享庫
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 複製應用程式二進位檔到容器中
COPY --from=builder /usr/src/qrcode-actix/target/release/qrcode-actix /usr/local/bin/qrcode-actix

# 指定容器應該如何執行您的應用程式
CMD ["qrcode-actix"]
