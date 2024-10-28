# Table of contents
- [Links](#links)
- [VHD shell](#vhd-shell)
    - [Setup the vhd file for testing](#setup-the-vhd-file-for-testing)
    - [Current work](#current-work)
    - [Debug](#debug)
    - [Notes](#notes)
    - [Block Allocation Table](#bat-block-allocation-table)
- [VHD NBD toolkit](#vhd-nbdkit-plugin)
- [QCow lib](#qcow-lib)
---

# Links

## VHD
- [VHD Specifications](https://github.com/libyal/libvhdi/blob/main/documentation/Virtual%20Hard%20Disk%20(VHD)%20image%20format.asciidoc)
- [Blktap vhd lib](https://github.com/xapi-project/blktap/tree/master/vhd/lib)

## Qcow
- [The Qcow2 Image format](https://www.talisman.org/~erlkonig/misc/qcow-image-format.html)
- [Qcow2 Specs](https://github.com/zchee/go-qcow2/blob/master/docs/specification.md)
- [Dirty Bitmaps & Incremental Backup](https://www.qemu.org/docs/master/interop/bitmaps.html)

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
- [ ] read the *test1.txt* in the RAW VHD file
- [ ] create a shell like to be able to get information from VHD file

```
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/vhd-shell`
== VHD FOOTER ==
next offset: 512
Disk size  : 6291456
Data size  : 6291456
Disk geometry: Cylinders 180/ Heads 4/ Sectors 17

== Dynamic Disk Header ==
Block table offset: 2048
Number of blocks  : 3
Block size        : 2097152

== BAT info ==
Block#0000 -> 0x00000005 : bitmap [0x00000a00-0x00000bff], data [0x00000c00-0x002009ff]
Block#0001 -> 0xffffffff : block is not allocated
Block#0002 -> 0x00001006 : bitmap [0x00200c00-0x00200dff], data [0x00200e00-0x00400bff]
```

## Debug

- You can use `hexdump` to view contents of the disk.
    - `hexdump -s 0xa00 -n 0x200 test.vhd`
- As we used *ext2* you can also inspect the data using `dumpe2fs`
```sh
sudo modprobe nbd
sudo qemu-nbd --connect=/dev/nbd0 ./test.vhd
sudo dumpe2fs /dev/nbd0p1
sudo qemu-nbd --disconnect /dev/nbd0
```

## Notes

- Blocks are by default 2M (0x200000)
- Each block is 4096 Sectors of 512 bytes
- At the beginning of each block you have a sector bitmap
    - A sector bitmap needs 4096 bits to track allocation of all sectors
        - for block that is not 2M: `size of bitmap (in bytes) = block size / ( 512 * 8 )`
        - sector bitmap is padded to 512 byte sector
    - So the first 512 bytes (4096 bits) are used a sector bitmap
    - It remains 2_096_640 bytes for data

### **BAT**: Block Allocation Table

- A block allocation table is only needed for dynamic and differential disk images
- A block allocation table consists of 32-bit entries
    - 0xFFFFFFFF => block is sparse or stored in parent
    - Otherwise  => it is the sector number where the data block starts
                    file offset = (entry * 512) + sector bitmap size
- *blockNumber* is used as index in BAT
- *Actual sector location* == BAT[BlockNumber] + BlockBitmapSectorCount + SectorInBlock
- Examples: I created a disk of 5M, so we need 3 blocks (10200 sectors)
    - If I write sector 1000
        - *blockNumber* = 1000/4096 = 0
        - *sectorInBlock* = 1000
    - If I write sector 5000
        - blockNumber = 5000/4096 = 1
        - sectorInBlock = 1096

# vhd nbdkit plugin

- Currently it is not using VHD lib.
- To build it: `cargo build -p vhd-nbdkit`
- Start the server: `nbdkit ./target/debug/libvhd_nbdkit.so -f -v`
- Now we can use a client:
    - `sudo qemu-nbd --connect=/dev/nbd0 nbd://127.0.0.1:10809`
    - `sudo qemu-nbd --disconnect /dev/nbd0`
- Next steps are to read/write into a VHD file

# Qcow Lib

- Let's do the same for Qcow. We will need to rename the project...
