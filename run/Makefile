include ../config.mk

PWD=$(shell pwd)
BASE_IMAGE_PATH := $(PWD)/../dist/$(QCOW_IMAGE_FILENAME)

IMAGE_PATH := $(PWD)/vda.img
CONFIG_DIR := $(PWD)/config
KERNEL_PATH := $(PWD)/../dist/vmlinuz-$(KERNEL_VERSION)
INITRD_PATH := $(PWD)/../dist/initrd.img
ROOTFS_PATH := $(PWD)/../dist/rootfs.iso
SSH_PORT := 10023

export IMAGE_PATH
export CONFIG_DIR
export KERNEL_PATH
export INITRD_PATH
export ROOTFS_PATH
export SSH_PORT

$(IMAGE_PATH): $(BASE_IMAGE_PATH)
	qemu-img create -f qcow2 -o backing_file=$<,backing_fmt=qcow2 $@

$(CONFIG_DIR):
	mkdir -p $@

$(BASE_IMAGE_PATH):
	$(MAKE) -C ../ dist-image

$(KERNEL_PATH):
	$(MAKE) -C ../ dist-kernel

$(INITRD_PATH):
	$(MAKE) -C ../ dist-initramfs

$(ROOTFS_PATH):
	$(MAKE) -C ../ dist-rootfs

.PHONY: run clean

run: $(IMAGE_PATH) $(CONFIG_DIR) $(KERNEL_PATH) $(INITRD_PATH) $(ROOTFS_PATH)
	./run.sh

clean:
	rm -f $(IMAGE_PATH)
	rm -rf $(CONFIG_DIR)