# NBDKit for VHD

- First step is to start from the ramdisk example
- Start the server: `nbdkit libvhd_nbdkit.so -f -v`
- Now we can use a client:
    - `sudo qemu-nbd --connect=/dev/nbd0 nbd://127.0.0.1:10809`
    - `sudo qemu-nbd --disconnect /dev/nbd0`
- Next steps are to read/write into a VHD file
