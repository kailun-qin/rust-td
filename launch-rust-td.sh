#!/bin/bash
now=$(date +"%m%d_%H%M")
LOGFILE=stdout.${now}.log

QEMU=/home/oem/tdvf-install/usr/local/bin/qemu-system-x86_64
BIOS=/home/oem/final.bin

$QEMU \
  -no-reboot -name debug-threads=on -enable-kvm -smp 1,sockets=1 -object tdx-guest,id=tdx,debug \
  -machine q35,accel=kvm,kvm-type=tdx,kernel_irqchip=split,guest-memory-protection=tdx -no-hpet \
  -cpu host,host-phys-bits,+invtsc \
  -device loader,file=$BIOS,id=fd0 \
  -m 2G -nographic -vga none | tee -a ${LOGFILE}
