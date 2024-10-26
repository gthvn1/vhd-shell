# vhd-shell

Goal: A shell-like to explore VHD file

# Setup the vhd file
- To create the file I used the Ocaml [vhd-tool](https://opam.ocaml.org/packages/vhd-tool/)
  - `vhd-tool create --size 5242880 test.vhd`
- Then I created a partition on it
```sh
sudo modprobe nbd
sudo qemu-nbd --connect=/dev/nbd0 ./test.vhd
sudo fdisk /dev/nbd0
sudo qemu-nbd --disconnect /dev/nbd0

```
