# Build in WSL: make
# Flash in Windows: make flash

TARGET = thumbv7em-none-eabihf
BIN = target/$(TARGET)/release/tiva_controller

all:
	cargo build --release

flash:
	openocd -f openocd.cfg -c "program $(BIN) verify reset exit"

clean:
	cargo clean

.PHONY: all flash clean
