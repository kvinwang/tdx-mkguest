# Makefile for TDX Guest Image Creation

# Variables
SCRIPT := ./create-td-image.sh
BUILD_DIR := build
OUTPUT_IMAGE := tdx-guest-ubuntu-24.04-generic.qcow2
INSTALL_DIR := $(shell pwd)/../dist

# Default target
all: $(OUTPUT_IMAGE)

# Create the TDX guest image
$(OUTPUT_IMAGE): $(SCRIPT)
	$(SCRIPT) -o $(OUTPUT_IMAGE)

# Clean up build artifacts
clean:
	rm -rf $(BUILD_DIR)
	rm -f $(OUTPUT_IMAGE)

# Force recreation of the image
force:
	$(SCRIPT) -o $(OUTPUT_IMAGE)

install: $(OUTPUT_IMAGE)
	mkdir -p $(INSTALL_DIR)
	cp -r $(OUTPUT_IMAGE) $(INSTALL_DIR)

.PHONY: all clean force help install
