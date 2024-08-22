# Makefile for TDX Guest Initramfs Creation

include ../config.mk

# Variables
MKINITRAMFS := ./mkinitramfs
OUTPUT_INITRD := initrd.img
INSTALL_DIR := $(shell pwd)/../dist

# Default target
all: $(OUTPUT_INITRD)

# Create the TDX guest initramfs
$(OUTPUT_INITRD): $(MKINITRAMFS)
	$(MKINITRAMFS) -v -d config -o $(OUTPUT_INITRD) $(KERNEL_VERSION)

# Install the initramfs
install: $(OUTPUT_INITRD)
	mkdir -p $(INSTALL_DIR)
	cp $(OUTPUT_INITRD) $(INSTALL_DIR)

# Clean up build artifacts
clean:
	rm -f $(OUTPUT_INITRD)

.PHONY: all clean install
