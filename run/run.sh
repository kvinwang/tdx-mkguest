#!/bin/bash

TD_IMG=${IMAGE_PATH:-${PWD}/vda.img}

SSH_PORT=${SSH_PORT:-10023}
PROCESS_NAME=qemu

INITRD=${INITRD_PATH:-${PWD}/../dist/initrd.img}
KERNEL=${KERNEL_PATH:-${PWD}/../dist/vmlinuz}
CDROM=${ROOTFS_PATH:-${PWD}/../dist/rootfs.iso}
BOOT=${BOOT:-rafs}
INTEGRITY=${INTEGRITY:-}
CONFIG_DIR=${CONFIG_DIR:-${PWD}/config}

if [ "${INTEGRITY}" == "1" ]; then
	INTEGRITY="hmac-sha256"
fi

ARGS="${ARGS} -kernel ${KERNEL}"
ARGS="${ARGS} -initrd ${INITRD}"
CMDLINE="root=/dev/vda1 ro console=tty1 console=ttyS0 boot=${BOOT} rootintegrity=${INTEGRITY} initimg=/dev/sr0"

echo INITRD=${INITRD}
echo ARGS=${ARGS}
echo TD_IMG=${TD_IMG}
echo CMDLINE=${CMDLINE}

sleep 2

qemu-system-x86_64 \
		   -accel kvm \
		   -m 2G -smp 8 \
		   -name ${PROCESS_NAME},process=${PROCESS_NAME},debug-threads=on \
		   -cpu host \
		   -machine q35,kernel_irqchip=split \
		   -nographic \
		   -nodefaults \
		   -chardev stdio,id=ser0,signal=on -serial chardev:ser0 \
		   -device virtio-net-pci,netdev=nic0_td -netdev user,id=nic0_td,hostfwd=tcp::${SSH_PORT}-:22 \
		   -drive file=${TD_IMG},if=none,id=virtio-disk0 -device virtio-blk-pci,drive=virtio-disk0 \
		   -cdrom ${CDROM} \
		   -virtfs local,path=${CONFIG_DIR},mount_tag=config,readonly=on,security_model=mapped,id=virtfs0 \
		   ${ARGS} \
		   -append "${CMDLINE}"
