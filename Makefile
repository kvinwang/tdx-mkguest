# TDX Guest Components: Image, Rootfs, and Initramfs

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

install: $(addprefix install-,$(COMPONENTS))

clean: $(addprefix clean-,$(COMPONENTS))

$(addprefix clean-,$(COMPONENTS)):
	$(MAKE) -C $(subst clean-,,$@) clean
