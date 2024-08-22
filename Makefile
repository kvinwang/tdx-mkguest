# TDX Guest Components: Image, Rootfs, and Initramfs

include config.mk

INSTALL_DIR := $(shell pwd)/dist
COMPONENTS := image rootfs initramfs

.PHONY: all $(COMPONENTS) install $(addprefix install-,$(COMPONENTS)) clean $(addprefix clean-,$(COMPONENTS))

all: $(COMPONENTS)

$(COMPONENTS):
	$(MAKE) -C $@

define install_component
install-$(1): $(1)
	$(MAKE) -C $(1) install INSTALL_DIR=$(INSTALL_DIR)
endef

$(foreach component,$(COMPONENTS),$(eval $(call install_component,$(component))))

install-kernel:
	mkdir -p $(INSTALL_DIR)
	cp /boot/vmlinuz-$(KERNEL_VERSION) $(INSTALL_DIR)

install: $(addprefix install-,$(COMPONENTS)) install-kernel

clean: $(addprefix clean-,$(COMPONENTS))

$(addprefix clean-,$(COMPONENTS)):
	$(MAKE) -C $(subst clean-,,$@) clean

prepare-kernel:
	apt install --yes linux-image-$(KERNEL_VERSION)
	chmod a+r /boot/vmlinuz-$(KERNEL_VERSION)

.PHONY: run clean
run:
	$(MAKE) -C run run

clean:
	$(MAKE) -C run clean