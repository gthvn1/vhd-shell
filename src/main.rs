use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open a VHD file in binary mode
    let file = File::open("test.vhd")?;

    // Read the VHD Footer into a buffer
    // There is a copy at the beginning of the file and the FOOTER is 512 bytes
    let mut buffer = Vec::new();
    file.take(512).read_to_end(&mut buffer)?;

    // https://github.com/libyal/libvhdi/blob/main/documentation/Virtual%20Hard%20Disk%20(VHD)%20image%20format.asciidoc
    // Check the signature
    if &buffer[0..8] == "conectix".as_bytes() {
        println!("Got the signature");
    } else {
        println!("Found {:?} instead of conectix", &buffer[0..8]);
    }

    Ok(())
}
