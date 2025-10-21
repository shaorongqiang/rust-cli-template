# 多阶段构建 Dockerfile for Rust CLI 模板项目
# 使用官方 Rust 镜像作为构建环境
FROM rust:1.75-slim as builder

# 设置工作目录
WORKDIR /app

# 安装必要的系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 Cargo.toml 和 Cargo.lock (如果存在)
COPY Cargo.toml ./

# 创建虚拟的 main.rs 来缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖 (缓存层)
RUN cargo build --release && rm -rf src

# 复制源代码
COPY src ./src

# 构建应用程序
RUN cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN groupadd -r appuser && useradd -r -g appuser appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rust-cli-template /app/rust-cli-template

# 创建必要的目录
RUN mkdir -p /app/logs /app/config && \
    chown -R appuser:appuser /app

# 切换到非 root 用户
USER appuser

# 暴露端口 (如果需要)
EXPOSE 8580

# 设置环境变量
ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /app/rust-cli-template --help || exit 1

# 默认命令
CMD ["./rust-cli-template"]
