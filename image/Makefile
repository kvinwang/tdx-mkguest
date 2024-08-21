# Makefile for TDX Guest Image Creation

# Variables
SCRIPT := ./create-td-image.sh
BUILD_DIR := build
OUTPUT_IMAGE := tdx-guest-ubuntu-24.04-generic.qcow2

# Default target
all: $(OUTPUT_IMAGE)

# Create the TDX guest image
$(OUTPUT_IMAGE): $(SCRIPT)
	@$(SCRIPT) -o $(OUTPUT_IMAGE)

# Clean up build artifacts
clean:
	@rm -rf $(BUILD_DIR)
	@rm -f $(OUTPUT_IMAGE)

# Force recreation of the image
force:
	@$(SCRIPT) -o $(OUTPUT_IMAGE)

# Help target
help:
	@echo "Available targets:"
	@echo "  all (default) - Create the TDX guest image"
	@echo "  clean         - Remove build artifacts and output image"
	@echo "  force         - Force recreation of the image"
	@echo "  help          - Show this help message"

.PHONY: all clean force help
