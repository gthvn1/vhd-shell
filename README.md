# vhd-shell
A shell to explore VHD file

# Setup the vhd file

```sh
sudo modprobe nbd
sudo qemu-nbd --connect=/dev/nbd0 ./test.vhd
sudo fdisk /dev/nbd0
sudo qemu-nbd --disconnect /dev/nbd0

```
