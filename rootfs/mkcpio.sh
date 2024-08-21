#!/bin/bash

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
IMAGE_PATH=$(realpath $1)
OUTPUT_IMAGE=$(realpath $2)

TMP_ROOTFS=${SCRIPT_DIR}/tmprootfs

if mountpoint -q ${TMP_ROOTFS}; then
    umount -l ${TMP_ROOTFS}
fi

rm -rf ${TMP_ROOTFS}
mkdir -p ${TMP_ROOTFS}
trap "rm -rf ${TMP_ROOTFS}" EXIT

./qcow2fuse -o fakeroot -o ro -p 1 ${IMAGE_PATH} ${TMP_ROOTFS}
trap "umount -l ${TMP_ROOTFS}" EXIT

cd ${TMP_ROOTFS} && find . | cpio -o --format=newc > ${OUTPUT_IMAGE}
