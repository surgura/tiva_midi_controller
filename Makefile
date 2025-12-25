# Build in WSL: make
# Flash in Windows: make flash

TARGET = thumbv7em-none-eabihf
ELF_RELEASE = target/$(TARGET)/release/tiva_controller
BIN = $(ELF_RELEASE).bin

all:
	cargo build --release
	arm-none-eabi-objcopy -O binary --only-section=.vector_table --only-section=.text --only-section=.rodata --only-section=.data --gap-fill=0xff $(ELF_RELEASE) $(BIN)

flash:
	openocd -f openocd.cfg -c "program $(BIN) 0x0 verify reset exit"

clean:
	cargo clean

.PHONY: all flash clean
