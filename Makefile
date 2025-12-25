# Build in WSL: make
# Flash in Windows: make flash
# Debug: make debug (starts OpenOCD), then in another terminal: make gdb

TARGET = thumbv7em-none-eabihf
ELF_RELEASE = target/$(TARGET)/release/tiva_controller
ELF_DEBUG = target/$(TARGET)/debug/tiva_controller
ELF = $(ELF_DEBUG)  # Use debug build for debugging
BIN = $(ELF_RELEASE).bin
# Try to find GDB - prefer ARM-specific, fallback to system GDB
GDB = $(shell which arm-none-eabi-gdb 2>/dev/null || which gdb 2>/dev/null || echo "gdb")
OPENOCD_PID = .openocd.pid

all:
	cargo build --release
	arm-none-eabi-objcopy -O binary --only-section=.vector_table --only-section=.text --only-section=.rodata --only-section=.data --gap-fill=0xff $(ELF_RELEASE) $(BIN)

# Build debug version with symbols
debug-build:
	cargo build

flash:
	openocd -f openocd.cfg -c "program $(BIN) 0x0 verify reset exit"

# Start OpenOCD in the background for debugging
# Note: May need sudo for USB access
debug:
	@echo "Starting OpenOCD GDB server on port 3333..."
	@echo "Note: If you get LIBUSB_ERROR_ACCESS, you may need to run: sudo make debug"
	@openocd -f openocd.cfg > .openocd.log 2>&1 & echo $$! > $(OPENOCD_PID)
	@echo "OpenOCD started (PID: $$(cat $(OPENOCD_PID)))"
	@echo "Waiting for OpenOCD to be ready..."
	@sleep 3
	@if ! kill -0 $$(cat $(OPENOCD_PID)) 2>/dev/null; then \
		echo ""; \
		echo "ERROR: OpenOCD process died. Check .openocd.log"; \
		tail -10 .openocd.log; \
		rm -f $(OPENOCD_PID); \
		exit 1; \
	fi
	@echo "OpenOCD is running (SRST errors are normal warnings)"
	@for i in 1 2 3 4 5; do \
		if command -v nc >/dev/null 2>&1 && nc -z localhost 3333 2>/dev/null; then \
			echo "OpenOCD GDB server is ready on port 3333!"; \
			break; \
		fi; \
		echo "Waiting for GDB server... ($$i/5)"; \
		sleep 1; \
	done
	@if command -v nc >/dev/null 2>&1 && ! nc -z localhost 3333 2>/dev/null; then \
		echo "WARNING: GDB server not responding on port 3333"; \
		echo "Check .openocd.log for details:"; \
		tail -15 .openocd.log; \
	else \
		echo "OpenOCD is running successfully!"; \
	fi
	@echo "Connect GDB with: make gdb"
	@echo "Or manually: $(GDB) $(ELF)"
	@echo "Then in GDB: target remote :3333"
	@echo "Stop OpenOCD with: make debug-stop"

# Stop OpenOCD
debug-stop:
	@if [ -f $(OPENOCD_PID) ]; then \
		kill $$(cat $(OPENOCD_PID)) 2>/dev/null || true; \
		rm -f $(OPENOCD_PID); \
		echo "OpenOCD stopped"; \
	else \
		echo "OpenOCD is not running"; \
	fi

# Connect GDB to OpenOCD
gdb:
	@if [ ! -f $(OPENOCD_PID) ]; then \
		echo "Error: OpenOCD is not running. Start it with: make debug"; \
		exit 1; \
	fi
	@if ! command -v $(GDB) >/dev/null 2>&1; then \
		echo "Error: GDB not found."; \
		echo "Install with: zypper install gdb"; \
		exit 1; \
	fi
	@if [ ! -f $(ELF) ]; then \
		echo "Error: Debug build not found. Run: make debug-build"; \
		exit 1; \
	fi
	@echo "Waiting for OpenOCD GDB server to be ready..."
	@for i in 1 2 3 4 5; do \
		if command -v nc >/dev/null 2>&1 && nc -z localhost 3333 2>/dev/null; then \
			echo "OpenOCD is ready!"; \
			break; \
		fi; \
		echo "Waiting... ($$i/5)"; \
		sleep 1; \
	done
	@echo "Using GDB: $(GDB)"
	@echo "Connecting to OpenOCD on port 3333..."
	@if echo "$(GDB)" | grep -q "arm-none-eabi"; then \
		$(GDB) $(ELF) -ex "target remote :3333" -ex "monitor reset halt" -ex "load"; \
	else \
		echo "Using system GDB - setting ARM architecture..."; \
		$(GDB) $(ELF) -ex "set architecture arm" -ex "target remote :3333" -ex "monitor reset halt" -ex "load"; \
	fi

# Clean build artifacts
clean:
	cargo clean
	rm -f $(OPENOCD_PID) .openocd.log

.PHONY: all flash debug debug-stop gdb clean
