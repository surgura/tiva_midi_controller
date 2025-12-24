# Makefile for TM4C123GXL LaunchPad
# Uses arm-none-eabi-gcc and OpenOCD

# Project name
PROJECT = tiva_controller

# Toolchain
CC = arm-none-eabi-gcc
OBJCOPY = arm-none-eabi-objcopy
OBJDUMP = arm-none-eabi-objdump
SIZE = arm-none-eabi-size
GDB = arm-none-eabi-gdb

# Target MCU
MCU = TM4C123GH6PM
CPU = cortex-m4
FPU = fpv4-sp-d16
FLOAT_ABI = hard

# Directories
SRC_DIR = src
BUILD_DIR = build

# Source files
SRCS = $(wildcard $(SRC_DIR)/*.c)
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%.o)

# Linker script
LDSCRIPT = tm4c123gh6pm.ld

# Compiler flags
CFLAGS = -mcpu=$(CPU) -mthumb -mfpu=$(FPU) -mfloat-abi=$(FLOAT_ABI)
CFLAGS += -O2 -g -gdwarf-2
CFLAGS += -Wall -Wextra -Werror
CFLAGS += -ffunction-sections -fdata-sections
CFLAGS += -D$(MCU)

# Linker flags
LDFLAGS = -mcpu=$(CPU) -mthumb -mfpu=$(FPU) -mfloat-abi=$(FLOAT_ABI)
LDFLAGS += -T$(LDSCRIPT)
LDFLAGS += -Wl,--gc-sections
LDFLAGS += -Wl,-Map=$(BUILD_DIR)/$(PROJECT).map
LDFLAGS += --specs=nosys.specs
LDFLAGS += -nostartfiles

# OpenOCD configuration
# For WSL: use Windows OpenOCD (uncomment the line below)
# OPENOCD = /mnt/c/openocd/bin/openocd.exe
OPENOCD = openocd
OPENOCD_CFG = openocd.cfg

# Targets
.PHONY: all clean flash flash-win debug gdb help size disasm

.DEFAULT_GOAL := all

help: ## Show this help message
	@echo "TM4C123GXL LaunchPad - Available targets:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Configuration:"
	@echo "  PROJECT     = $(PROJECT)"
	@echo "  MCU         = $(MCU)"
	@echo "  CPU         = $(CPU)"
	@echo "  OPENOCD_CFG = $(OPENOCD_CFG)"

all: $(BUILD_DIR)/$(PROJECT).elf $(BUILD_DIR)/$(PROJECT).bin $(BUILD_DIR)/$(PROJECT).hex ## Build firmware (elf, bin, hex)
	@$(SIZE) $(BUILD_DIR)/$(PROJECT).elf

$(BUILD_DIR)/$(PROJECT).elf: $(OBJS)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(LDFLAGS) -o $@ $^
	$(OBJDUMP) -D $@ > $(BUILD_DIR)/$(PROJECT).lst

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c -o $@ $<

$(BUILD_DIR)/$(PROJECT).bin: $(BUILD_DIR)/$(PROJECT).elf
	$(OBJCOPY) -O binary $< $@

$(BUILD_DIR)/$(PROJECT).hex: $(BUILD_DIR)/$(PROJECT).elf
	$(OBJCOPY) -O ihex $< $@

clean: ## Remove build artifacts
	rm -rf $(BUILD_DIR)

flash: $(BUILD_DIR)/$(PROJECT).elf ## Build and flash firmware to device
	$(OPENOCD) -f $(OPENOCD_CFG) -c "init; reset halt; flash write_image erase $(BUILD_DIR)/$(PROJECT).elf; verify_image $(BUILD_DIR)/$(PROJECT).elf; reset run; shutdown"

debug: ## Start OpenOCD debug server
	$(OPENOCD) -f $(OPENOCD_CFG)

gdb: $(BUILD_DIR)/$(PROJECT).elf ## Connect GDB to OpenOCD debug server
	$(GDB) -ex "target extended-remote :3333" \
	       -ex "monitor reset halt" \
	       -ex "load" \
	       $(BUILD_DIR)/$(PROJECT).elf

# WSL-specific: Flash using Windows OpenOCD
# Requires OpenOCD installed on Windows (e.g., via MSYS2 or standalone)
OPENOCD_WIN = /mnt/c/msys64/mingw64/bin/openocd.exe
WIN_PROJECT_PATH = $(shell wslpath -w $(CURDIR))

flash-win: $(BUILD_DIR)/$(PROJECT).elf ## Flash via Windows OpenOCD (for WSL)
	$(OPENOCD_WIN) -f "$(WIN_PROJECT_PATH)\\openocd.cfg" \
		-c "program $(WIN_PROJECT_PATH)\\build\\$(PROJECT).elf verify reset exit"

size: $(BUILD_DIR)/$(PROJECT).elf ## Show firmware size breakdown
	@$(SIZE) -A -x $(BUILD_DIR)/$(PROJECT).elf

disasm: $(BUILD_DIR)/$(PROJECT).elf ## Open disassembly listing
	@less $(BUILD_DIR)/$(PROJECT).lst
