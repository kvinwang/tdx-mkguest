# Makefile for TDX Guest Initramfs Creation

# Variables
MKINITRAMFS := ./mkinitramfs
OUTPUT_INITRD := initrd.img
KERNEL_VERSION := 6.8.0-40-generic

# Default target
all: $(OUTPUT_INITRD)

# Create the TDX guest initramfs
$(OUTPUT_INITRD): $(MKINITRAMFS)
	$(MKINITRAMFS) -v -d config -o $(OUTPUT_INITRD) $(KERNEL_VERSION)

# Clean up build artifacts
clean:
	rm -f $(OUTPUT_INITRD)

.PHONY: all clean force help
