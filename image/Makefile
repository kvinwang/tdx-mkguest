# Makefile for TDX Guest Image Creation

include ../config.mk

export QCOW_IMAGE_FILENAME

# Variables
SCRIPT := ./create-td-image.sh
BUILD_DIR := build
OUTPUT_IMAGE := $(QCOW_IMAGE_FILENAME)
INSTALL_DIR := $(shell pwd)/../dist

# Default target
all: $(OUTPUT_IMAGE)

# Create the TDX guest image
$(OUTPUT_IMAGE): $(SCRIPT) tdxctl
	$(SCRIPT) -o $(OUTPUT_IMAGE)

tdxctl: ../target/release/tdxctl
	cp ../target/release/tdxctl .

../target/release/tdxctl:
	cargo build --release -p tdxctl

# Clean up build artifacts
clean:
	rm -rf $(BUILD_DIR)
	rm -f $(OUTPUT_IMAGE)

# Force recreation of the image
force:
	rm -f $(OUTPUT_IMAGE)
	$(MAKE) $(OUTPUT_IMAGE)

install: $(OUTPUT_IMAGE)
	mkdir -p $(INSTALL_DIR)
	cp $(OUTPUT_IMAGE) $(INSTALL_DIR)

.PHONY: all clean force install
