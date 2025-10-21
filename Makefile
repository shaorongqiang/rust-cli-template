# Rust CLI 模板项目 Makefile
# 提供常用的构建、测试、清理等操作

# 项目信息
PROJECT_NAME = rust-cli-template
BINARY_NAME = $(PROJECT_NAME)
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
DEBUG_DIR = $(TARGET_DIR)/debug

# 默认目标
.DEFAULT_GOAL := help

# 颜色定义
RED = \033[0;31m
GREEN = \033[0;32m
YELLOW = \033[0;33m
BLUE = \033[0;34m
PURPLE = \033[0;35m
CYAN = \033[0;36m
WHITE = \033[0;37m
NC = \033[0m # No Color

# 帮助信息
.PHONY: help
help: ## 显示帮助信息
	@echo "$(CYAN)Rust CLI 模板项目 - 可用命令:$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""

# 构建相关
.PHONY: build
build: ## 构建项目 (debug 模式)
	@echo "$(BLUE)构建项目 (debug 模式)...$(NC)"
	cargo build
	@echo "$(GREEN)构建完成!$(NC)"

.PHONY: build-release
build-release: ## 构建项目 (release 模式)
	@echo "$(BLUE)构建项目 (release 模式)...$(NC)"
	cargo build --release
	@echo "$(GREEN)Release 构建完成!$(NC)"

.PHONY: run
run: ## 运行项目
	@echo "$(BLUE)运行项目...$(NC)"
	cargo run

.PHONY: run-release
run-release: build-release ## 运行 release 版本
	@echo "$(BLUE)运行 release 版本...$(NC)"
	./$(RELEASE_DIR)/$(BINARY_NAME)

# 测试相关
.PHONY: test
test: ## 运行所有测试
	@echo "$(BLUE)运行测试...$(NC)"
	cargo test
	@echo "$(GREEN)测试完成!$(NC)"

.PHONY: test-verbose
test-verbose: ## 运行测试并显示输出
	@echo "$(BLUE)运行测试 (详细输出)...$(NC)"
	cargo test -- --nocapture
	@echo "$(GREEN)测试完成!$(NC)"

.PHONY: test-release
test-release: ## 在 release 模式下运行测试
	@echo "$(BLUE)在 release 模式下运行测试...$(NC)"
	cargo test --release
	@echo "$(GREEN)Release 测试完成!$(NC)"

# 代码质量
.PHONY: check
check: ## 检查代码 (不构建)
	@echo "$(BLUE)检查代码...$(NC)"
	cargo check
	@echo "$(GREEN)代码检查完成!$(NC)"

.PHONY: clippy
clippy: ## 运行 clippy 代码检查
	@echo "$(BLUE)运行 clippy 检查...$(NC)"
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)Clippy 检查完成!$(NC)"

.PHONY: fmt
fmt: ## 格式化代码
	@echo "$(BLUE)格式化代码...$(NC)"
	cargo fmt
	@echo "$(GREEN)代码格式化完成!$(NC)"

.PHONY: fmt-check
fmt-check: ## 检查代码格式
	@echo "$(BLUE)检查代码格式...$(NC)"
	cargo fmt -- --check
	@echo "$(GREEN)代码格式检查完成!$(NC)"

# 清理相关
.PHONY: clean
clean: ## 清理构建文件
	@echo "$(BLUE)清理构建文件...$(NC)"
	cargo clean
	@echo "$(GREEN)清理完成!$(NC)"

.PHONY: clean-all
clean-all: clean ## 清理所有文件 (包括日志和配置文件)
	@echo "$(BLUE)清理所有文件...$(NC)"
	rm -rf logs/
	rm -f *.toml
	rm -f *.pem
	rm -f *.log
	@echo "$(GREEN)完全清理完成!$(NC)"

# 文档相关
.PHONY: doc
doc: ## 生成文档
	@echo "$(BLUE)生成文档...$(NC)"
	cargo doc --no-deps
	@echo "$(GREEN)文档生成完成!$(NC)"

.PHONY: doc-open
doc-open: ## 生成并打开文档
	@echo "$(BLUE)生成并打开文档...$(NC)"
	cargo doc --no-deps --open
	@echo "$(GREEN)文档已打开!$(NC)"

# 依赖管理
.PHONY: update
update: ## 更新依赖
	@echo "$(BLUE)更新依赖...$(NC)"
	cargo update
	@echo "$(GREEN)依赖更新完成!$(NC)"

.PHONY: audit
audit: ## 审计依赖安全性
	@echo "$(BLUE)审计依赖安全性...$(NC)"
	cargo audit
	@echo "$(GREEN)安全审计完成!$(NC)"

# 安装相关
.PHONY: install
install: build-release ## 安装到系统
	@echo "$(BLUE)安装到系统...$(NC)"
	cp $(RELEASE_DIR)/$(BINARY_NAME) /usr/local/bin/
	@echo "$(GREEN)安装完成!$(NC)"

.PHONY: uninstall
uninstall: ## 从系统卸载
	@echo "$(BLUE)从系统卸载...$(NC)"
	rm -f /usr/local/bin/$(BINARY_NAME)
	@echo "$(GREEN)卸载完成!$(NC)"

# 开发工具
.PHONY: watch
watch: ## 监视文件变化并自动构建
	@echo "$(BLUE)监视文件变化...$(NC)"
	cargo watch -x build

.PHONY: watch-test
watch-test: ## 监视文件变化并自动运行测试
	@echo "$(BLUE)监视文件变化并运行测试...$(NC)"
	cargo watch -x test

.PHONY: watch-run
watch-run: ## 监视文件变化并自动运行
	@echo "$(BLUE)监视文件变化并运行...$(NC)"
	cargo watch -x run

# 配置相关
.PHONY: config
config: ## 生成默认配置文件
	@echo "$(BLUE)生成默认配置文件...$(NC)"
	cargo run -- --config config.toml
	@echo "$(GREEN)配置文件已生成: config.toml$(NC)"

.PHONY: config-dev
config-dev: ## 生成开发环境配置文件
	@echo "$(BLUE)生成开发环境配置文件...$(NC)"
	LOG_LEVEL=debug cargo run -- --config config-dev.toml
	@echo "$(GREEN)开发配置文件已生成: config-dev.toml$(NC)"

# 发布相关
.PHONY: package
package: build-release ## 打包 release 版本
	@echo "$(BLUE)打包 release 版本...$(NC)"
	tar -czf $(BINARY_NAME)-release.tar.gz -C $(RELEASE_DIR) $(BINARY_NAME)
	@echo "$(GREEN)打包完成: $(BINARY_NAME)-release.tar.gz$(NC)"

.PHONY: dist
dist: clean build-release package ## 创建发布包
	@echo "$(GREEN)发布包创建完成!$(NC)"

# 调试相关
.PHONY: debug
debug: ## 使用调试信息运行
	@echo "$(BLUE)使用调试信息运行...$(NC)"
	cargo run

.PHONY: profile
profile: build-release ## 性能分析
	@echo "$(BLUE)性能分析...$(NC)"
	perf record ./$(RELEASE_DIR)/$(BINARY_NAME)
	@echo "$(GREEN)性能分析完成!$(NC)"

# 完整工作流
.PHONY: all
all: fmt clippy test build-release ## 运行完整的构建流程
	@echo "$(GREEN)完整构建流程完成!$(NC)"

.PHONY: ci
ci: fmt-check clippy test ## CI/CD 流程
	@echo "$(GREEN)CI/CD 流程完成!$(NC)"

# 信息显示
.PHONY: info
info: ## 显示项目信息
	@echo "$(CYAN)项目信息:$(NC)"
	@echo "  项目名称: $(PROJECT_NAME)"
	@echo "  二进制文件: $(BINARY_NAME)"
	@echo "  目标目录: $(TARGET_DIR)"
	@echo "  Release 目录: $(RELEASE_DIR)"
	@echo "  Debug 目录: $(DEBUG_DIR)"
	@echo ""
	@echo "$(CYAN)Rust 版本:$(NC)"
	@rustc --version
	@echo ""
	@echo "$(CYAN)Cargo 版本:$(NC)"
	@cargo --version

# 默认目标
.PHONY: default
default: help
