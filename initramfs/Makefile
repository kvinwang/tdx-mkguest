# Makefile for TDX Guest Initramfs Creation

include ../config.mk

# Variables
MKINITRAMFS := ./mkinitramfs
OUTPUT_INITRD := initrd.img
INSTALL_DIR := $(shell pwd)/../dist
TDX_KO = $(shell pwd)/mod-tdx-guest/tdx-guest.ko

export TDX_KO

# Default target
all: $(OUTPUT_INITRD)

# Create the TDX guest initramfs
$(OUTPUT_INITRD): $(MKINITRAMFS) $(TDX_KO)
	$(MKINITRAMFS) -v -d config -o $(OUTPUT_INITRD) $(KERNEL_VERSION)

ko: $(TDX_KO)

$(TDX_KO):
	make -C /lib/modules/$(KERNEL_VERSION)/build M=`pwd`/mod-tdx-guest

# Install the initramfs
install: $(OUTPUT_INITRD)
	mkdir -p $(INSTALL_DIR)
	cp $(OUTPUT_INITRD) $(INSTALL_DIR)

# Clean up build artifacts
clean:
	rm -f $(OUTPUT_INITRD) *.ko
	make -C /lib/modules/$(KERNEL_VERSION)/build M=`pwd`/mod-tdx-guest clean

.PHONY: all clean install
