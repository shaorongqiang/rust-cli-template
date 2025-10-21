# Rust CLI 模板项目

一个功能完整的 Rust CLI 应用程序模板，集成了配置管理、日志记录、数据库连接、JWT 令牌认证和服务器功能。

## 🚀 功能特性

- **配置管理**: 支持 TOML 配置文件和命令行参数
- **环境变量**: 完整的环境变量支持
- **错误处理**: 使用 anyhow 进行错误处理

## 📦 依赖项

- `anyhow`: 错误处理
- `clap`: 命令行参数解析
- `tracing`: 日志记录
- `tokio`: 异步运行时
- `jwt-simple`: JWT 令牌处理
- `serde`: 序列化/反序列化
- `toml`: TOML 配置文件支持

## 🛠️ 安装和构建

### 前置要求

- Rust 1.70+ (推荐使用最新稳定版)

### 构建项目

```bash
# 克隆项目
git clone https://github.com/shaorongqiang/rust-cli-template
cd rust-cli-template

# 构建项目
cargo build --release

# 运行项目
cargo run
```

## 📖 使用方法

### 基本用法

```bash
# 使用默认配置运行
cargo run

# 指定配置文件
cargo run -- --config config.toml

# 查看帮助信息
cargo run -- --help
```

### 配置文件

首次运行时，程序会自动生成默认配置文件：

```toml
# 日志配置
[log]
dir = "logs"
level = "info"

# 服务器配置
[server]
listen = "0.0.0.0:8580"
access_url = "http://127.0.0.1:8580"
allowed_origins = []

# 数据库配置
[db]
url = "postgres://postgres:123456@127.0.0.1:5432/postgres"
enable_logging = false
min_connections = null
max_connections = null
connect_timeout = null
idle_timeout = null

# JWT 令牌配置
[token]
access_key_path = "access_key.pem"
access_token_expired_minutes = 10
refresh_key_path = "refresh_key.pem"
refresh_token_expired_minutes = 120
refresh_token_max_expired_minutes = 1440
```

### 命令行参数

#### 日志选项
- `-D, --log-dir <DIR>`: 日志文件目录
- `-L, --log-level <LEVEL>`: 日志级别 (debug, info, warn, error)

#### 服务器选项
- `-l, --server-listen <ADDRESS>`: 服务器监听地址
- `--server-access-url <URL>`: 服务器访问 URL
- `--server-allowed-origins <ORIGINS>`: 允许的 CORS 源

#### 数据库选项
- `--database-url <URL>`: 数据库连接 URL
- `--database-init`: 初始化数据库
- `--database-enable-logging`: 启用数据库日志
- `--database-min-connections <NUM>`: 最小连接数
- `--database-max-connections <NUM>`: 最大连接数
- `--database-connect-timeout <SECONDS>`: 连接超时
- `--database-idle-timeout <SECONDS>`: 空闲超时

#### JWT 令牌选项
- `--token-access-key-path <PATH>`: 访问令牌私钥路径
- `--token-access-token-expired-minutes <MINUTES>`: 访问令牌过期时间
- `--token-refresh-key-path <PATH>`: 刷新令牌私钥路径
- `--token-refresh-token-expired-minutes <MINUTES>`: 刷新令牌过期时间
- `--token-refresh-token-max-expired-minutes <MINUTES>`: 刷新令牌最大过期时间

### 环境变量

所有配置选项都支持通过环境变量设置：

```bash
# 日志配置
export LOG_DIR=./logs
export LOG_LEVEL=debug

# 服务器配置
export SERVER_LISTEN=0.0.0.0:8080
export SERVER_ACCESS_URL=http://localhost:8080

# 数据库配置
export DATABASE_URL=postgres://user:password@localhost:5432/mydb

# JWT 配置
export TOKEN_ACCESS_KEY_PATH=./keys/access_key.pem
export TOKEN_REFRESH_KEY_PATH=./keys/refresh_key.pem
```

## 🏗️ 项目结构

```
src/
├── main.rs              # 主程序入口
└── config/              # 配置模块
    ├── mod.rs           # 模块声明
    ├── config.rs        # 主配置结构
    ├── log.rs           # 日志配置
    ├── server.rs        # 服务器配置
    ├── database.rs      # 数据库配置
    └── token.rs         # JWT 令牌配置
```

## 🔧 开发

### 代码风格

项目使用严格的 Rust 代码检查：

```rust
#![deny(
    warnings,
    unused_crate_dependencies,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]
```

## 📝 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📞 支持

如果您遇到任何问题或有任何建议，请：

1. 查看 [Issues](../../issues) 页面
2. 创建新的 Issue
3. 联系维护者

---

**注意**: 这是一个模板项目，您可以根据自己的需求进行修改和扩展。