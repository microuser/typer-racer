# Makefile for Typer Racer

# Default target: build everything and launch web
all: build test wasm up

# mall: clean, then build, test, wasm, web
mall: down clean build test wasm up

# Build native Rust binary
build:
	cargo build

# Run tests
test:
	cargo test

# Build WebAssembly package (using wasm-pack by default)
wasm:
	RUSTFLAGS='--cfg=web_sys_unstable_apis' wasm-pack build --target web

# Launch local web server (using trunk) in daemon mode, save PID
web:
	trunk serve

# Alias: up target (same as web)
up:
	@echo "Checking if port 8080 (trunk serve) is open..."
	@if lsof -i :8080 | grep LISTEN; then \
	  echo "Port 8080 is open. Stopping process..."; \
	  kill -9 $$(lsof -ti:8080); \
	  echo "Process on port 8080 stopped."; \
	else \
	  echo "Port 8080 is not open."; \
	fi
	(trunk serve & echo $$! > .webserver.pid)
# Take down the web server

down:
	@echo "Checking if port 8080 (trunk serve) is open..."
	@if lsof -i :8080 | grep LISTEN; then \
	  echo "Port 8080 is open. Stopping process..."; \
	  kill -9 $$(lsof -ti:8080); \
	  echo "Process on port 8080 stopped."; \
	else \
	  echo "Port 8080 is not open."; \
	fi
	

# Clean build artifacts
clean:
	cargo clean
	wasm-pack clean || true

# Install native binary to ~/.bin (macOS style)
install:
	@mkdir -p $$HOME/.bin
	@cp target/debug/typer-racer $$HOME/.bin/typer-racer
	@echo "Installed typer-racer to $$HOME/.bin/typer-racer"
	@if ! echo $$PATH | grep -q "$${HOME}/.bin"; then \
	  echo "Add 'export PATH=\$$HOME/.bin:\$$PATH' to your shell profile to use 'typer-racer' from anywhere."; \
	fi

# echo Use make all to build, test, wasm, and launch the web server.

# Setup instructions (not a make target):
# 1. Install Rust: https://rustup.rs/
# 2. Install wasm-pack: cargo install wasm-pack
# 3. Install trunk: cargo install trunk
# 4. Run make all to start everything for web development.
