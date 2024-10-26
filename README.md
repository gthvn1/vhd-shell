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
Confirmed that sig is "conectix"
next offset: 512
Disk size  : 6291456
Data size  : 6291456
Disk geometry: Cylinders 180/ Heads 4/ Sectors 17

== Dynamic Disk Header ==
Confirmed that sig is "cxsparse"
Block table offset: 2048
Number of blocks  : 3
Block size        : 2097152
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

- A block allocation table is only needed for dynamic and differential disk images
- A block allocation table consists of 32-bit entries
    - 0xFFFFFFFF => block is sparse or stored in parent
    - Otherwise  => it is the sector number where the data block starts
                    file offset = (entry * 512) + sector bitmap size

## Links

- [VHD Specifications](https://github.com/libyal/libvhdi/blob/main/documentation/Virtual%20Hard%20Disk%20(VHD)%20image%20format.asciidoc)
