# Makefile for TM4C123GXL LaunchPad (Rust)
# Uses cargo and OpenOCD
#
# Workflow:
#   1. Build in WSL:     make release
#   2. Flash in Windows: make flash

# Project name
PROJECT = tiva_controller

# Target
TARGET = thumbv7em-none-eabihf

# Binary locations
CARGO_BIN = target/$(TARGET)/release/$(PROJECT)
CARGO_BIN_DEBUG = target/$(TARGET)/debug/$(PROJECT)

# OpenOCD configuration
OPENOCD = openocd
OPENOCD_CFG = openocd.cfg

# Targets
.PHONY: all clean flash flash-debug build-flash debug gdb help size release

.DEFAULT_GOAL := all

help: ## Show this help message
	@echo "TM4C123GXL LaunchPad (Rust) - Available targets:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Configuration:"
	@echo "  PROJECT = $(PROJECT)"
	@echo "  TARGET  = $(TARGET)"

all: ## Build firmware (debug mode)
	cargo build
	@arm-none-eabi-size $(CARGO_BIN_DEBUG)

release: ## Build firmware (release mode, optimized)
	cargo build --release
	@arm-none-eabi-size $(CARGO_BIN)

clean: ## Remove build artifacts
	cargo clean

flash: ## Flash pre-built firmware (run from Windows after 'make release' in WSL)
	$(OPENOCD) -f $(OPENOCD_CFG) -c "program $(CARGO_BIN) verify reset exit"

flash-debug: ## Flash pre-built debug firmware (run from Windows after 'make' in WSL)
	$(OPENOCD) -f $(OPENOCD_CFG) -c "program $(CARGO_BIN_DEBUG) verify reset exit"

build-flash: release ## Build and flash (WSL only, requires cargo)
	$(OPENOCD) -f $(OPENOCD_CFG) -c "program $(CARGO_BIN) verify reset exit"

debug: ## Start OpenOCD debug server
	$(OPENOCD) -f $(OPENOCD_CFG)

gdb: all ## Connect GDB to OpenOCD debug server
	arm-none-eabi-gdb -x openocd.gdb $(CARGO_BIN_DEBUG)

gdb-release: release ## Connect GDB to OpenOCD debug server (release build)
	arm-none-eabi-gdb -x openocd.gdb $(CARGO_BIN)

size: release ## Show firmware size breakdown
	@arm-none-eabi-size -A -x $(CARGO_BIN)

objdump: release ## Generate disassembly listing
	arm-none-eabi-objdump -D $(CARGO_BIN) > target/$(PROJECT).lst
	@echo "Disassembly written to target/$(PROJECT).lst"

# Convenience targets
check: ## Run cargo check
	cargo check

clippy: ## Run clippy linter
	cargo clippy

fmt: ## Format code with rustfmt
	cargo fmt

doc: ## Generate documentation
	cargo doc --open
