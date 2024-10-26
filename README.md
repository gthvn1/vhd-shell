# vhd-shell

**Goal**: A shell-like to explore VHD file

## Setup the vhd file for testing

- To create the file I used the Ocaml [vhd-tool](https://opam.ocaml.org/packages/vhd-tool/)
  - `vhd-tool create --size 5242880 test.vhd`
- Then I use *nbd* to access the file as a block device and create a partition
on it using fdisk:
```sh
sudo modprobe nbd
sudo qemu-nbd --connect=/dev/nbd0 ./test.vhd
sudo fdisk /dev/nbd0
... [ create one primary partition ] 
```
- In order to create a filesystem: `sudo mkfs.ext2 /dev/nbd0p1`
- So I can mount it: `sudo mount /dev/nbd0p1 /mnt/`
- And as root create a file: `echo "Hello, Sailor!" > /mnt/test1.txt`
- Finally unmount it: `sudo umount /mnt`
- And disconnect it: `sudo qemu-nbd --disconnect /dev/nbd0`
- Now I have a quiet small *test.vhd* file that has a partition, a file system and a file
on it. It's all we need to play with VHD and understand it.

## Current work

- [x] We can read the footer and the dynamic disk header
- [] read the *test1.txt* in the RAW VHD file
- [] create a shell like to be able to get information from VHD file

```
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/vhd-shell`
Confirmed that sig is "conectix"
next offset: 512
Confirmed that sig is "cxsparse"
Block table offset: 2048
```

## Links

- [VHD Specifications](https://github.com/libyal/libvhdi/blob/main/documentation/Virtual%20Hard%20Disk%20(VHD)%20image%20format.asciidoc)
