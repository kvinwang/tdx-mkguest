
# Makefile for TDX Guest Initramfs Creation

# Variables
OUTPUT_IMAGE := rootfs.iso
IMAGE_PATH := ../image/tdx-guest-ubuntu-24.04-generic.qcow2
ISO_ROOTDIR := isoroot
CPIO_IMAGE := $(ISO_ROOTDIR)/rootfs.cpio

# Default target
all: $(OUTPUT_IMAGE)

# Create the TDX guest initramfs
$(OUTPUT_IMAGE): $(CPIO_IMAGE)
	mkisofs -o $(OUTPUT_IMAGE) --max-iso9660-filenames -input-charset utf-8 $(ISO_ROOTDIR)

$(CPIO_IMAGE): $(IMAGE_PATH)
	mkdir -p $(ISO_ROOTDIR)
	./mkcpio.sh $(IMAGE_PATH) $(CPIO_IMAGE)

$(IMAGE_PATH):
	$(MAKE) -C ../image

# Clean up build artifacts
clean:
	rm -f $(OUTPUT_INITRD)

.PHONY: all clean
