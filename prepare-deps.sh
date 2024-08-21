#!/bin/bash

apt update

# install required tools
apt install --yes qemu-utils libguestfs-tools virtinst genisoimage libvirt-daemon-system

# to allow virt-customize to have name resolution, dhclient should be available
# on the host system. that is because virt-customize will create an appliance (with supermin)
# from the host system and will collect dhclient into the appliance
apt install --yes isc-dhcp-client

chmod a+r /boot/vmlinuz-$(uname -r)
